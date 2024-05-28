/*
 * @Author: plucky
 * @Date: 2022-06-05 23:25:16
 * @LastEditTime: 2023-11-08 10:25:04
 */

use std::time::Duration;

use hello_tonic::pb::HelloRequest;
use hello_tonic::pb::greeter_client::GreeterClient;
use tonic::metadata::{MetadataValue, Ascii};
use tonic::transport::Endpoint;
// use tower::timeout::Timeout;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 普通的连接
    // let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // 使用超时的中间件
    let channel = Endpoint::from_static("http://127.0.0.1:50051")
        .timeout(Duration::from_secs(3))
        .connect_timeout(Duration::from_secs(5))
        .connect().await?;

    // 使用超时的中间件
    // let channel = tower::timeout::Timeout::new(channel, Duration::from_millis(1000));
    // 普通的客户端
    // let mut client = GreeterClient::new(channel);
    
    // 带token验证的客户端
    let token: MetadataValue<Ascii> = "Bearer some-auth-token".parse()?;
    let mut client = GreeterClient::with_interceptor(channel, move |mut req: tonic::Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    // let response =  tokio::time::timeout(Duration::from_secs(3), client.say_hello(request)).await?;
    println!("RESPONSE={:?}", response);

   

    Ok(())
}