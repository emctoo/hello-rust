use std::fmt::Error;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{Extension, Router};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use w2::hub;

pub struct State {
    channels: hub::ChannelManager,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsocketMessage {
    pub content: String,
    pub kind: String,
}

mod y {
    pub fn hi() -> String { "hi".to_string() }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "srv=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let channels = hub::ChannelManager::new();
    channels.new_channel("global".into(), None).await;
    channels.new_channel("room".into(), None).await;

    let state = Arc::new(State { channels });

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    // build our application with a single route
    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", get(websocket_handler))
        .layer(Extension(state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with axum on localhost:5000
    info!("listening on :3000 ...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

pub async fn websocket_handler(ws: WebSocketUpgrade, Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

#[derive(Debug, Serialize, Deserialize)]
struct Agent {
    pub id: u32,
    pub session_id: u32,
}

impl Agent {
    fn new(id: u32, session_id: u32) -> Self {
        Self { id, session_id }
    }
}

async fn check_auth_header(_: &str) -> Result<Agent, Error> { Ok(Agent::new(0, 0)) }

async fn websocket(stream: WebSocket, state: Arc<State>) {
    let (mut socket_tx, mut socket_rx) = stream.split();
    // TODO wait it for some time (30 seconds?) for auth payload, or close the connection

    let mut agent = Agent { id: 0, session_id: 0 };
    while let Some(Ok(Message::Text(token))) = socket_rx.next().await {
        match check_auth_header(&token).await {
            Ok(Agent { id, session_id }) => {
                agent.id = id;
                agent.session_id = session_id;
                info!("authed, user info updated, {}/{}", id, session_id);
                break;
            }
            Err(_) => {
                warn!("auth failure, closing connection ...");
                socket_tx.send(Message::Text("auth failure, closing socket".to_string())).await.unwrap();
                socket_tx.close().await.unwrap();
                warn!("auth failure, connection closed");
                return;
            }
        }
    }

    // init for authenticated user
    state.channels.register(agent.session_id.to_string(), None).await;

    // join user to global room
    state.channels.join_channel("global".into(), agent.session_id.to_string()).await.unwrap();
    info!("user {} added to global channel", agent.id);

    // get receiver for user that get message from all rooms
    let mut user_receiver = state.channels.get_user_receiver(agent.session_id.to_string()).await.unwrap();

    // spawn a task to get message from channel and send it to user
    let mut send_task = tokio::spawn(async move {
        info!("relay message from channel to users ...");
        while let Ok(data) = user_receiver.recv().await {
            debug!("message from channel: {}", data);
            socket_tx.send(Message::Text(data)).await.unwrap();
        }
    });

    let rec_state = state.clone();

    // spawn a task to get message from user and handle things
    let mut socket_receive_task = tokio::spawn(async move {
        info!("recv task for user launched");

        while let message = socket_rx.next().await {
            match message {
                Some(Ok(Message::Text(data))) => {
                    info!("message received: {}", data);

                    if data.starts_with("join") {
                        // we can join rooms and receiver gets message from that room
                        rec_state.channels.join_channel("room".into(), agent.session_id.to_string()).await.unwrap();
                    }

                    if data.starts_with("send") {
                        // we can send message to a specific room
                        rec_state.channels.send_message_to_channel("room".into(), data).await.unwrap();
                    } else {
                        rec_state.channels.send_message_to_channel("global".into(), data).await.unwrap();
                    }
                }
                _ => {
                    error!("error when receiving messages from socket");
                    break;
                }
            }
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = &mut send_task => socket_receive_task.abort(),
        _ = &mut socket_receive_task => send_task.abort(),
    }

    // after connection closed you have to call this so all things gets removed safely
    state.channels.deregister(agent.session_id.to_string()).await;
    info!("connection closed");
}
