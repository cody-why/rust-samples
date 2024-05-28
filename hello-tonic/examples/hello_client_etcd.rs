/*
 * @Author: plucky
 * @Date: 2022-06-05 23:25:16
 * @LastEditTime: 2023-11-10 16:46:46
 */
// etcd discovery


use etcd_client::ConnectOptions;
use hello_tonic::{EtcdDiscovery, pb::{HelloRequest, greeter_client::GreeterClient}};
use tonic::metadata::MetadataValue;
// use tower::timeout::Timeout;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    // 使用etcd服务发现
    let opt = ConnectOptions::new().with_user("tonic_user", "789789");
    let mut discover = EtcdDiscovery::connect(["127.0.0.1:2379"], Some(opt)).await?;
    discover.service_discover("/hello").await?;
    
    let channel = discover.get_service("/hello/1").unwrap();
     // 普通的客户端
    // let mut client = GreeterClient::new(channel);
    // 带token验证的客户端
    let token: MetadataValue<_> = "Bearer some-auth-token".parse()?;
    let mut client = GreeterClient::with_interceptor(channel, move |mut req: tonic::Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
    

    for _ in 0..3 {
        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });

        let response = client.say_hello(request).await?;
        println!("RESPONSE={:?}", response);

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
    }
   

    Ok(())
}


