// #![allow(unused)]

use axum::{
    routing::get,
     Router, Extension,
};
use dashmap::DashMap;
use tracing::{info};
use std::{env, net::SocketAddr, sync::Arc};
// use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tower_http::{trace::TraceLayer};
mod handler;
mod test;

pub struct AppState {
    group_list: DashMap<String, broadcast::Sender<String>>,
    nc: nats::asynk::Connection,
}
// ws://127.0.0.1:8088/websocket/group1/1
#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug"); 
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    // use nats broadcast messages to group clients
    let nats_host = env::var("NATS_HOST").unwrap_or("127.0.0.1:4222".to_string());
    let nc = match nats::asynk::connect(&nats_host).await {
        Ok(nc) => nc,
        Err(e) => panic!("{:?}", e),
    };

    // let group_list = Mutex::new(HashMap::new());
    let group_list = DashMap::new();
    let app_state = Arc::new(AppState { group_list, nc });

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
        .layer(TraceLayer::new_for_http())
        // .layer(
        //     TraceLayer::new_for_http()
        //         .on_request(|request: &Request<_>, _span: &Span| {
        //             debug!("started {} {}", request.method(), request.uri().path())
        //         })
        //         .on_response(|_response: &Response<_>, latency: Duration, _span: &Span| {
        //             debug!("response generated in {:?}", latency)
        //         })
        //         .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
        //             debug!("sending {} bytes", chunk.len())
        //         })
        //         .on_eos(
        //             |_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
        //                 debug!("stream closed after {:?}", stream_duration)
        //             },
        //         )
        //         .on_failure(
        //             |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
        //                 debug!("something went wrong: {:?}", error)
        //             },
        //         ),
        // )
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
