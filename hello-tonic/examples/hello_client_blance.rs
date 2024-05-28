/*
 * @Author: plucky
 * @Date: 2023-11-06 08:34:16
 * @LastEditTime: 2023-11-08 10:18:08
 */

 // ginepro dns负载均衡的客户端


use anyhow::Context;
use ginepro::LoadBalancedChannel;
use tonic::metadata::MetadataValue;

use hello_tonic::pb::{greeter_client::GreeterClient, HelloRequest};
 

 #[tokio::main]
 async fn main() -> anyhow::Result<()> {
    // k8s dns负载均衡的客户端
    let channel = LoadBalancedChannel::builder(("localhost", 50051_u16))
        // 尝试连接的每个新端点设置超时
        .timeout(std::time::Duration::from_secs(10))
        //对于流失率较高的系统，可以缩短探测间隔。
        .dns_probe_interval(std::time::Duration::from_secs(5))
        .channel()
        .await
        .context("failed to construct LoadBalancedChannel")?;
 
    //  let mut client = GreeterClient::new(channel);
    // 带token验证的客户端
    let token: MetadataValue<_> = "Bearer some-auth-token".parse()?;
    let mut client = GreeterClient::with_interceptor(channel, move |mut req: tonic::Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
 
     let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
 
    let response = client.say_hello(request).await?;
 
     println!("RESPONSE={:?}", response);
 
     Ok(())
 }