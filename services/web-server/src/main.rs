use std::{convert::Infallible, time::Duration};

use axum::{
    Router,
    extract::State,
    response::{Sse, sse::Event},
    routing::get,
};
use futures_util::Stream;
use rpc_router::RpcNotification;
use serde_json::json;
use tokio::{
    net::TcpListener,
    sync::broadcast::{self, channel},
};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

async fn sse_handler(
    State(app_state): State<AppState>,
) -> Sse<impl Stream<Item = core::result::Result<Event, Infallible>>> {
    let rx = app_state.tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok(evt) => Some(Ok(Event::default().event("sse-event").data(evt))),
        Err(_) => None,
    });

    Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(1)))
}

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let (tx, _) = channel::<String>(100);

    let state = AppState { tx: tx.clone() };

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;

            let org_uuid = Uuid::new_v4();
            let user_uuid = Uuid::new_v4();

            let notif = RpcNotification {
                method: "org_user_kick".into(),
                params: Some(json!({ "org_id": org_uuid, "user_id": user_uuid })),
            };

            let json = serde_json::to_string(&notif).unwrap();
            if let Err(err) = tx.send(json.clone()) {
                error!("Couldn't send the error ->> {err}")
            }

            info!("EVT sent ->> {json}");
        }
    });

    let router = Router::new()
        .route("/sse", get(sse_handler))
        .with_state(state)
        .layer(CorsLayer::very_permissive());

    let listener = TcpListener::bind("0.0.0.0:8081").await?;

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
