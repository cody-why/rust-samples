/***
 * @Author: plucky
 * @Date: 2022-09-08 23:50:51
 * @LastEditTime: 2022-09-09 00:16:48
 * @Description: 
 */

use lazy_static::lazy_static;
use volo_demo::layer::LogLayer;
use std::net::SocketAddr;

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .layer_inner(LogLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let req = volo_gen::volo::example::GetItemRequest { id: 1024 };
    let resp = CLIENT.clone().get_item(req).await;
    match resp {
        Ok(info) => tracing::info!("{:?}", info),
        Err(e) => tracing::error!("{:?}", e),
    }
}