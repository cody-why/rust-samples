/*
 * @Author: plucky
 * @Date: 2022-08-31 19:47:55
 * @LastEditTime: 2022-11-10 23:50:12
 * @Description: 
 */

// #![allow(unused)]

use axum::{
    routing::get,
     Router, Extension,
};
use tracing::{info};
use std::{env, net::SocketAddr, sync::Arc};

use crate::state::AppState;
// use tokio::sync::Mutex;
// use tower_http::{trace::TraceLayer, classify::ServerErrorsFailureClass};
mod handler;
mod test;
pub mod state;


// ws://127.0.0.1:8088/websocket/group1/1
#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info"); 
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    // use nats broadcast messages to group clients
    let nats_host = env::var("NATS_HOST").unwrap_or("127.0.0.1:4222".to_string());
    let nc = match nats::asynk::connect(&nats_host).await {
        Ok(nc) => nc,
        Err(e) => panic!("{:?}", e),
    };

    // let group_list = Mutex::new(HashMap::new());
    let app_state = Arc::new(AppState::new(nc));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8088));

    let mut s_task = gen_server_task(app_state.clone(), addr);
    let mut nc_task = gen_nc_task(app_state);
    info!("listening on {}", addr);
    tokio::select! {
        _ = (&mut nc_task) => s_task.abort(),
        _ = (&mut s_task) => nc_task.abort(),
    }
}

// ws://127.0.0.1:8088/websocket/group1/1
fn app(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/websocket/:group_id/:user_id",
            get(handler::ws::handler),
        )
        .layer(Extension(app_state))
        // .layer(TraceLayer::new_for_http())
        // .layer(TraceLayer::new_for_http()
        //     .on_request(|_request: &Request<Body>, _span: &Span| {
        //         info!("started {} {}", _request.method(), _request.uri().path());
        //     })
        //     .on_response(())
        //     .on_body_chunk(())
        //     .on_eos(())
        //     .on_failure(|error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
        //         info!("something went wrong {error} {latency:?}")
        // }))

}

fn gen_nc_task(app_state: Arc<AppState>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let sub = match app_state.nc.subscribe("*").await {
            Ok(sub) => sub,
            Err(e) => panic!("{:?}", e),
        };
        while let Some(msg) = sub.next().await {
            // let mut group_list = app_state.group_list.lock().await;

            if let Some(group) = app_state.group_list.get_mut(&msg.subject) {
                let converted: String = match String::from_utf8(msg.data) {
                    Ok(v) => v,
                    Err(e) => e.to_string(),
                };
                info!("{}", converted);
                let _drop = group.send(converted);
            }
        }
    })
}

fn gen_server_task(app_state: Arc<AppState>, addr: SocketAddr) -> tokio::task::JoinHandle<()> {
    let app = app(app_state);
    tokio::spawn(async move {
        if let Err(e) = axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
        {
            panic!("{:?}", e)
        }
    })
}
