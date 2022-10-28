/*
 * @Author: plucky
 * @Date: 2022-10-28 18:02:53
 * @LastEditTime: 2022-10-28 20:43:56
 * @Description: 
 */

use dubbo::Dubbo;
use dubbo_config::RootConfig;
use dubbo_demo::protos::echo::echo_server::{register_server};
use dubbo_demo::echo::server::*;

#[tokio::main]
async fn main() {
    std::env::set_var("DUBBO_CONFIG_PATH", "dubbo_echo.yaml");
    register_server(EchoServerImpl {
        name: "echo".to_string(),
    });
    // let server = EchoServerImpl::default();
    // let s = EchoServer::<EchoServerImpl>::with_filter(server, FakeFilter {});
    // dubbo::protocol::triple::TRIPLE_SERVICES
    //     .write()
    //     .unwrap()
    //     .insert(
    //         "grpc.examples.echo.Echo".to_string(),
    //         dubbo::utils::boxed_clone::BoxCloneService::new(s),
    //     );

    // Dubbo::new().start().await;
    Dubbo::new()
        .with_config({
            let r = RootConfig::new();
            // r.test_config();
            match r.load() {
                Ok(config) => config,
                Err(_err) => panic!("err: {:?}", _err), // response was droped
            }
            // r.service.remove("kafka");
            
        })
        .start()
        .await;
}

#[derive(Clone)]
pub struct FakeFilter {}

// impl Filter for FakeFilter {
//     fn call(&mut self, req: Request<()>) -> Result<Request<()>, dubbo::status::Status> {
//         println!("server fake filter: {:?}", req.metadata);
//         Ok(req)
//     }
// }