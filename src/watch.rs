use crate::CompileParams;
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
use tokio::{fs, net::TcpListener, sync::Notify};

use std::{error::Error, net::SocketAddr, path::PathBuf, sync::Arc};

pub struct SharedState {
    pub port: u16,
    pub address: String,
    pub output: PathBuf,
    pub changed: Notify,
}

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

    let state = Arc::new(SharedState { port, address, output, changed: Notify::new() });

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

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| match res {
        Ok(event) => {
            if let Modify(Data(DataChange::Content)) = event.kind {
                if event.paths.iter().any(|p| p.extension().unwrap() == "pdf") {
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
    axum::serve(listener, router).await?;

    Ok(())
}

pub async fn root(State(state): State<Arc<SharedState>>) -> Html<String> {
    include_str!("../assets/index.html")
        .replace("{addr}", &state.address)
        .replace("{port}", &state.port.to_string())
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
