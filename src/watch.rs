use std::{
    error::Error, fs::remove_file, future::IntoFuture, net::SocketAddr, path::PathBuf, sync::Arc,
};

use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use notify::{
    event::{DataChange, ModifyKind::Data},
    Event,
    EventKind::Modify,
    RecursiveMode, Watcher,
};
use tokio::{fs, net::TcpListener, select, sync::Notify};

use crate::CompileParams;

pub struct SharedState {
    pub port: u16,
    pub address: String,
    pub input: PathBuf,
    pub output: PathBuf,
    pub changed: Notify,
    pub shutdown: Notify,
}

// list of supported extensions
const EXTENSIONS: [&str; 16] = [
    "cbor", "csv", "gif", "htm", "html", "jpeg", "jpg", "json", "png", "svg", "toml", "txt", "typ",
    "xml", "yaml", "yml",
];

/// Starts a web server that serves the output PDF file, while watching for changes in the input
/// Typst file and recompiles when a change is detected.
///
/// # Arguments
///
/// - `params` - CompileParams struct.
/// - `open` - Whether to open the output PDF file in the default browser.
pub async fn watch(params: &CompileParams, open: bool) -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(&addr).await?;
    let address = listener.local_addr()?.ip().to_string();
    let port = listener.local_addr()?.port();

    let input = params.input.clone();
    let output = params.output.clone();
    let params = params.clone();

    match crate::compile(&params) {
        Ok(duration) => {
            println!("Initial compilation succeeded in {duration:?}. Watching for changes...")
        }
        Err(why) => eprintln!("{why}"),
    }

    let state = Arc::new(SharedState {
        port,
        address,
        input: input.clone(),
        output,
        changed: Notify::new(),
        shutdown: Notify::new(),
    });
    let state_handler = Arc::clone(&state);
    let state_selector = Arc::clone(&state);

    let router = Router::new()
        .route("/", get(root))
        .route("/target.pdf", get(pdf))
        .route("/listen", get(listen))
        .with_state(Arc::clone(&state));
    println!("Listening on {}:{}", state.address, state.port);

    if open {
        match open::that_detached(format!("http://{}:{}", state.address, state.port)) {
            Ok(_) => println!("Opened in default browser"),
            Err(why) => eprintln!("{why}"),
        }
    }

    tokio::spawn(async move {
        println!("Press Ctrl+C to exit");
        async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to register handler for Ctrl+C");
        }
        .await;
        state_handler.shutdown.notify_waiters();
    });

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| match res {
        Ok(event) => {
            if let Modify(Data(DataChange::Content)) = event.kind {
                let changed = !event
                    .paths
                    .iter()
                    .filter_map(|p| p.extension())
                    .map(|e| e.to_string_lossy().to_lowercase())
                    .filter(|e| EXTENSIONS.contains(&e.as_str()))
                    .collect::<Vec<_>>()
                    .is_empty();
                if !changed {
                    return;
                }
                print!("Change detected. Recompiling...");
                match crate::compile(&params) {
                    Ok(duration) => println!("compilation succeeded in {duration:?}"),
                    Err(why) => eprintln!("{why}"),
                }
                state.changed.notify_one()
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;
    watcher.watch(input.parent().unwrap(), RecursiveMode::Recursive)?;
    let server = axum::serve(listener, router).into_future();

    select! {
        _ = server => {}
        _ = state_selector.shutdown.notified() => {
            println!("\nShutting down...");
            watcher.unwatch(input.parent().unwrap())?;
            remove_file(&state_selector.output)?;
        }
    }

    println!("Bye!");
    Ok(())
}

pub async fn root(State(state): State<Arc<SharedState>>) -> Html<String> {
    include_str!("../assets/index.html")
        .replace("{addr}", &state.address)
        .replace("{port}", &state.port.to_string())
        .replace("{input}", &state.input.display().to_string())
        .into()
}

pub async fn pdf(State(state): State<Arc<SharedState>>) -> impl IntoResponse {
    Response::builder()
        .header("Content-Type", "application/pdf")
        .body(Body::from(match fs::read(&state.output).await {
            Ok(data) => data,
            Err(why) => panic!("{:#?}", why),
        }))
        .unwrap()
}

pub async fn listen(
    State(state): State<Arc<SharedState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handler(socket, state))
}

async fn handler(mut socket: WebSocket, state: Arc<SharedState>) {
    loop {
        state.changed.notified().await;
        _ = socket.send(Message::Text("refresh".into())).await;
    }
}
