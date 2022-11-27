/*
 * @Author: plucky
 * @Date: 2022-09-08 22:54:31
 * @LastEditTime: 2022-11-20 18:56:17
 * @Description: 
 */
// #![feature(type_alias_impl_trait)]

use std::net::SocketAddr;

use volo_demo::{S, layer::LogLayer};



#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    // let addr = volo::net::Address::from(addr);

    volo_gen::volo::example::ItemServiceServer::new(S)
        .layer(LogLayer)
        .run(addr)
        .await
        .unwrap();
}
