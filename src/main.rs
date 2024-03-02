use self::meta::app_version;
use axum::extract::State as AxState;
use axum::routing::get;
use socketioxide::SocketIo;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info, Level};
use tracing_subscriber::{FmtSubscriber, Registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing_tree::{HierarchicalLayer};

mod event;
mod handlers;
pub mod id;
mod meta;
mod pokemon;
mod state;
mod dto;

async fn handler(AxState(io): AxState<SocketIo>) {
    info!("handler called");
    let _ = io.emit("hello", "world");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_env_filter("ppapp=debug,socketioxide=info,engineioxide=info")
        .finish();
    //     .with(HierarchicalLayer::new(2));

    // let subscriber = Registry::default().with(HierarchicalLayer::new(2).with_ansi(true));
    tracing::subscriber::set_global_default(subscriber)?;

    let (layer, io) = SocketIo::builder()
        .with_state(state::Users::default())
        .with_state(state::Sessions::default())
        .with_state(state::RoomState::default())
        .build_layer();

    io.ns("/", handlers::on_connection);

    info!("PPApp {}", app_version());

    let app = axum::Router::new()
        .route("/", get(|| async { app_version() }))
        .route("/hello", get(handler))
        .with_state(io)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    let port = 3010u16;
    let host = "0.0.0.0";
    info!(name: "starting", port, "Starting server");

    let listener = TcpListener::bind((host, port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
