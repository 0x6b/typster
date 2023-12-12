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

pub async fn start_server(params: &CompileParams) -> Result<(), Box<dyn Error>> {
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

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| match res {
        Ok(event) => {
            if let Modify(Data(DataChange::Content)) = event.kind {
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

    watcher.watch(&input, RecursiveMode::NonRecursive)?;
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
