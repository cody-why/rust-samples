/***
 * @Author: plucky
 * @Date: 2022-09-01 11:03:21
 * @LastEditTime: 2022-09-02 21:08:57
 * @Description: 
 */

use axum::{
    extract,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::Path,
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use tracing::info;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

use crate::AppState;


#[derive(Deserialize, Clone)]
pub struct Params {
    user_id: String,
    group_id: String,
}

pub async fn handler(
    Path(params): Path<Params>,
    ws: WebSocketUpgrade,
    extract::Extension(state): extract::Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state, params))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>, params: Params) {
    let (sender, mut receiver) = stream.split();
    let sender = Arc::new(Mutex::new(sender));
    let broadcast_sender = sender.clone();

    let tx = setup_sender(&state, &params.group_id).await;
    let user_id = params.user_id.clone();
    let group_id = params.group_id.clone();

    let nc = state.nc.clone();

    info!("{} join group {}", user_id, group_id);
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            match message.clone() {
                Message::Text(message) => {
                    let _drop = nc
                        .publish(&group_id, format!("{}: {}", user_id, message))
                        .await;
                }
                Message::Binary(_) => {}
                Message::Ping(ping) => {
                    if sender.lock().await.send(Message::Pong(ping)).await.is_err() {
                        break;
                    }
                }
                Message::Pong(_) => {}
                Message::Close(_) => {
                    
                }
            }
        }
    });

    let mut send_task = tokio::spawn(async move {
        let mut rx = tx.clone().subscribe();
        while let Ok(message) = rx.recv().await {
            if broadcast_sender
                .lock()
                .await
                .send(Message::Text(message))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    let group_id = params.group_id;

    info!("{} leave group {}", params.user_id, group_id);
    // let mut group_list = state.group_list.lock().await;
    let mut count = 0;
    // Locking behaviour by get
    if let Some(group) = state.group_list.get(&group_id) {
        count = group.receiver_count();
    }

    if count == 1 {
        state.group_list.remove(&group_id);
        info!("remove group {}", group_id);
    }
    
}

async fn setup_sender(state: &AppState, group_id: &str) -> broadcast::Sender<String> {
    // let mut group_list = state.group_list.lock().await;
    let group_id = group_id.to_string();

    state.group_list
        .entry(group_id)
        .or_insert_with(|| broadcast::channel(100).0)
        .clone()
}