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
    response::IntoResponse, http::{ HeaderMap},
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use tracing::{info, debug};
use std::sync::Arc;
use tokio::sync::{ Mutex};

use crate::AppState;


#[derive(Deserialize, Clone)]
pub struct Params {
    user_id: String,
    group_id: String,
}

pub async fn handler(
    Path(params): Path<Params>,
    header:HeaderMap,
    ws: WebSocketUpgrade,
    extract::Extension(state): extract::Extension<Arc<AppState>>,
) -> impl IntoResponse {
    debug!("websocket connect: {:?}", header);
    ws.on_upgrade(|socket| websocket(socket, state, params))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>, params: Params) {
    let (sender, mut receiver) = stream.split();
    let sender = Arc::new(Mutex::new(sender));
    let sender2 = sender.clone();

    let user_id = params.user_id.clone();
    let group_id = params.group_id.clone();
    info!("{} join group {}", user_id, group_id);

    let tx = state.setup_sender(&group_id);


    let nc = state.nc.clone();

    // 接收到消息,发送到广播通道
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

    // 接收广播消息,发送到客户端
    let mut send_task = tokio::spawn(async move {
        let mut rx = tx.clone().subscribe();
        while let Ok(message) = rx.recv().await {
            if sender2.lock().await
                .send(Message::Text(message)).await
                .is_err(){
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
    let count = state.remove_group(&group_id);

    if count {
        info!("remove group {}", group_id);
    }
    
}

