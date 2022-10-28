/*
 * @Author: plucky
 * @Date: 2022-10-28 11:32:58
 * @LastEditTime: 2022-10-28 20:43:38
 * @Description: 
 */

use dubbo::invocation::{Request, Response};
use dubbo_config::RootConfig;

use async_trait::async_trait;
use dubbo_demo::protos::greeter::{greeter_server::{Greeter, register_server}, GreeterRequest, GreeterReply};


#[tokio::main]
async fn main() {
    std::env::set_var("DUBBO_CONFIG_PATH", "dubbo.yaml");
    register_server(GreeterServerImpl {
        name: "greeter".to_string(),
    });

    // Dubbo::new().start().await;
    dubbo::Dubbo::new()
        .with_config({
            let r = RootConfig::new();
            match r.load() {
                Ok(config) => config,
                Err(_err) => panic!("err: {:?}", _err), // response was droped
            }
        })
        .start()
        .await;
}

#[allow(dead_code)]
#[derive(Default, Clone)]
struct GreeterServerImpl {
    name: String,
}

// #[async_trait]
#[async_trait]
impl Greeter for GreeterServerImpl {
    async fn greet(
        &self,
        request: Request<GreeterRequest>,
    ) -> Result<Response<GreeterReply>, dubbo::status::Status> {
        println!("GreeterServer::greet {:?}", request.metadata);

        Ok(Response::new(GreeterReply {
            message: "hello, dubbo-rust".to_string(),
        }))
    }
}