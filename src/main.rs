mod state;
mod meta;
mod event;
mod handlers;

use axum::routing::get;
use axum::extract::State as AxState;
use socketioxide::{
    extract::{Data, SocketRef, State},
    SocketIo,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::field::DebugValue;
use tracing::{info, debug, Level};
use tracing_subscriber::FmtSubscriber;
use crate::event::{Event, Messages};
use self::{meta::app_version};

// async fn on_connect(socket: SocketRef) {
//     info!(id = socket.id.to_string(), "Socket connected");
//
//     socket.on(
//         Event::Join,
//         |socket: SocketRef, Data::<String>(room), store: State<state::AppStore>| async move {
//             info!(room, "Received join");
//             let _ = socket.leave_all();
//             let _ = socket.join(room.clone());
//
//             debug!(id = debug(socket.id), "Getting user...");
//             store.get_user(&socket.id).await;
//
//             let users = store.get_users().await;
//             debug!(count = users.len(), "Sending users...");
//             let _ = socket.emit(Event::Users, event::Users { users });
//
//             let messages = store.get_messages(&room).await;
//             debug!(count = messages.len(), "Sending messages...");
//             let _ = socket.emit(Event::Messages, event::Messages { messages });
//         },
//     );
//
//     socket.on(
//         Event::Message,
//         |socket: SocketRef, Data::<event::MessageIn>(data), store: State<state::AppStore>| async move {
//             info!(data = debug(&data), "Received message");
//
//             let response = state::Message {
//                 text: data.text,
//                 user: socket.id.to_string(),
//                 date: chrono::Utc::now(),
//             };
//
//             store.insert_message(&data.room, response.clone()).await;
//
//             let _ = socket.within(data.room).emit("message", response);
//         },
//     );
//
//     socket.on(
//         Event::UpdateUser,
//         |socket: SocketRef, Data::<event::UserIn>(data), store: State<state::AppStore>| async move {
//             info!(data = debug(&data), "Received user update");
//
//             let user = state::User {
//                 name: data.name,
//             };
//
//             store.update_user(&socket.id, &user).await;
//
//             let _ = socket.within(data.room).emit(Event::UserUpdated, event::EntityUpdate {
//                 id: socket.id.to_string(),
//                 entity_type: "user",
//                 update: user,
//             });
//         },
//     )
// }

async fn handler(AxState(io): AxState<SocketIo>) {
    info!("handler called");
    let _ = io.emit("hello", "world");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)

        .finish()
    )?;

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

    let port = 3000u16;
    let host = "0.0.0.0";
    info!(name: "starting", port, "Starting server");

    let listener = TcpListener::bind((host, port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}