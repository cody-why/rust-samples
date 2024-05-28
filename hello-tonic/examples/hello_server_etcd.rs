/*
 * @Author: plucky
 * @Date: 2022-06-05 23:06:39
 * @LastEditTime: 2023-11-10 18:31:03
 */

use etcd_client::ConnectOptions;
use hello_tonic::EtcdRegister;
use hello_tonic::greeter::MyGreeter;
use tonic::{transport::Server, Request, Status};

use hello_tonic::pb::greeter_server::GreeterServer;

use tracing_subscriber::fmt::time::OffsetTime;


    
#[allow(unused)]
// 拦截器, 验证token
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    // tracing::info!("intercept: {:?}", req);
    let token = "Bearer some-auth-token";
    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}


async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051";
    
    // 使用etcd注册服务
    let opt = ConnectOptions::new().with_user("tonic_user", "789789");
    let mut register = EtcdRegister::connect(["127.0.0.1:2379"], Some(opt)).await?;
    register.lease_grant(30, 10).await?;
    register.put("/hello/1", format!("http://{addr}")).await?;
    
    let greeter = MyGreeter::default();
    //  let svc = GreeterServer::new(greeter);
    // 使用拦截器
    let svc = GreeterServer::with_interceptor(greeter, check_auth);


    tracing::info!("GreeterServer listening on: {}", addr);
    
    Server::builder()
        // 使用拦截器
        // .layer(tonic::service::interceptor(check_auth))
        .add_service(svc)
        .serve(addr.parse()?)
        .await?;

    Ok(())
}


fn main(){
    tracing_subscriber::fmt()
        .with_env_filter("info,hello_tonic=info")
        .with_timer(OffsetTime::local_rfc_3339().unwrap())
        .init();
    
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async_main()).unwrap();
}