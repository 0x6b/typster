use std::{
    error::Error, fmt::Display, fs::remove_file, future::IntoFuture, net::SocketAddr,
    path::PathBuf, sync::Arc,
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
use log::{error, info};
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
    pub fitting_type: FittingType,
}

// list of supported extensions
const EXTENSIONS: [&str; 16] = [
    "cbor", "csv", "gif", "htm", "html", "jpeg", "jpg", "json", "png", "svg", "toml", "txt", "typ",
    "xml", "yaml", "yml",
];

/// Starts a web server that serves the output PDF file, while watching for changes in the input
/// Typst file and recompiles when a change is detected.
///
///Changes for `typ` file, along with files with extension `cbor`, `csv`, `gif`, `htm`, `html`,
/// `jpeg`, `jpg`, `json`, `png`, `svg`, `toml`, `txt`, `xml`, `yaml`, and `yml` in the same
/// directory, recursively, will be watched. This is inspired by [ItsEthra/typst-live](https://github.com/ItsEthra/typst-live/).
///
/// # Arguments
///
/// - `params` - [`CompileParams`] struct.
/// - `open` - Whether to open the output PDF file with the default browser once after the server
///   launches.
/// - `app` - Open the output PDF file with the given application
///
/// # Example
///
/// Following is an example of how to use the `watch` function:
///
///```no_run
/// let rt = tokio::runtime::Runtime::new().unwrap();
/// let params = typster::CompileParams {
///     input: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///         .join("examples")
///         .join("sample.typ"),
///     output: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///         .join("examples")
///         .join("sample.pdf"),
///     font_paths: vec!["assets".into()],
///     dict: vec![("input".to_string(), "value".to_string())],
///     ppi: None,
///     package_path: None,
///     package_cache_path: None,
/// };
///
/// rt.block_on(async {
///     if let Err(error) = typster::watch(&params, true, None, Some(typster::FittingType::Width)).await {
///         eprintln!("Server error: {}", error)
///     }
/// });
/// ```
pub async fn watch(
    params: &CompileParams,
    open: bool,
    app: Option<&str>,
    fitting_type: Option<FittingType>,
) -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(&addr).await?;
    let address = listener.local_addr()?.ip().to_string();
    let port = listener.local_addr()?.port();

    let input = params.input.clone();
    let output = params.output.clone();
    let params = params.clone();

    match crate::compile(&params) {
        Ok(duration) => {
            info!("Initial compilation succeeded in {duration:?}. Watching for changes...")
        }
        Err(why) => error!("{why}"),
    }

    let state = Arc::new(SharedState {
        port,
        address,
        input: input.clone(),
        output,
        changed: Notify::new(),
        shutdown: Notify::new(),
        fitting_type: fitting_type.unwrap_or_default(),
    });
    let state_handler = Arc::clone(&state);
    let state_selector = Arc::clone(&state);

    let router = Router::new()
        .route("/", get(root))
        .route("/target.pdf", get(pdf))
        .route("/listen", get(listen))
        .with_state(Arc::clone(&state));
    info!("Listening on {}:{}", state.address, state.port);

    if open {
        if let Some(app) = app {
            match open::with_detached(format!("http://{}:{}", state.address, state.port), app) {
                Ok(_) => info!("Opened in default browser"),
                Err(why) => error!("{why}"),
            }
        } else {
            match open::that_detached(format!("http://{}:{}", state.address, state.port)) {
                Ok(_) => info!("Opened in default browser"),
                Err(why) => error!("{why}"),
            }
        }
    }

    tokio::spawn(async move {
        info!("Press Ctrl+C to exit");
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
                info!("Change detected. Recompiling...");
                match crate::compile(&params) {
                    Ok(duration) => info!("compilation succeeded in {duration:?}"),
                    Err(why) => error!("{why}"),
                }
                state.changed.notify_one()
            }
        }
        Err(e) => error!("watch error: {:?}", e),
    })?;
    watcher.watch(input.parent().unwrap(), RecursiveMode::Recursive)?;
    let server = axum::serve(listener, router).into_future();

    select! {
        _ = server => {}
        _ = state_selector.shutdown.notified() => {
            info!("Shutting down...");
            watcher.unwatch(input.parent().unwrap())?;
            remove_file(&state_selector.output)?;
        }
    }

    info!("Bye!");
    Ok(())
}

pub async fn root(State(state): State<Arc<SharedState>>) -> Html<String> {
    include_str!("../assets/index.html")
        .replace("{addr}", &state.address)
        .replace("{port}", &state.port.to_string())
        .replace("{input}", &state.input.display().to_string())
        .replace("{fitting_type}", &state.fitting_type.to_string())
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

/// Fitting type for the PDF output (Google Chrome only, maybe)
#[derive(Debug, Clone, Default)]
pub enum FittingType {
    /// Fit to page
    Page,
    /// Fit to width
    #[default]
    Width,
    /// Fit to height
    Height,
}

impl From<&str> for FittingType {
    fn from(value: &str) -> Self {
        match value {
            "page" => FittingType::Page,
            "width" => FittingType::Width,
            "height" => FittingType::Height,
            _ => FittingType::Page,
        }
    }
}

impl Display for FittingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // https://chromium.googlesource.com/chromium/src/+/6363f8da6aae63abedc87f60b629585f10bd8940/chrome/browser/resources/pdf/open_pdf_params_parser.js#61
        match self {
            FittingType::Page => write!(f, "fit"),
            FittingType::Width => write!(f, "fith"),
            FittingType::Height => write!(f, "fitv"),
        }
    }
}
