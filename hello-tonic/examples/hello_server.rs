/*
 * @Author: plucky
 * @Date: 2022-06-05 23:06:39
 * @LastEditTime: 2023-11-10 18:31:10
 */

use hello_tonic::greeter::MyGreeter;
use tonic::{transport::Server, Request, Status};

use hello_tonic::pb::greeter_server::GreeterServer;



    
#[allow(unused)]
// 拦截器, 验证token
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    // tracing::info!("intercept: {:?}", req);
    let token = "Bearer some-auth-token";
    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
    // Ok(req)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let addr = "127.0.0.1:50051".parse()?;
    let greeter = MyGreeter::default();
    // 使用拦截器
    let svc = GreeterServer::with_interceptor(greeter, check_auth);


    tracing::info!("GreeterServer listening on: {}", addr);
    
    Server::builder()
        // .trace_fn(|_| tracing::info_span!("helloworld_server"))
        // 使用拦截器
        // .layer(tonic::service::interceptor(check_auth))
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}

