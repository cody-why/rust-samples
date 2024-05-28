/*
 * @Author: plucky
 * @Date: 2023-11-05 10:51:37
 * @LastEditTime: 2023-11-05 12:34:35
 */

use std::net::SocketAddr;

use volo_grpc::server::{Server, ServiceBuilder};
use volo_grpc_demo::{S, layer::LogLayer};


#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    Server::new()
        .add_service(ServiceBuilder::new(volo_gen::volo::example::ItemServiceServer::new(S)).build())
        .layer(LogLayer)
        .run(addr)
        .await
        .unwrap();
}
