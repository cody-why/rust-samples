/*
 * @Author: plucky
 * @Date: 2022-06-05 23:25:16
 * @LastEditTime: 2023-11-08 10:23:42
 */
// MyChannel 实现的客户端,根据延时来切换服务

use std::sync::Arc;

use hello_tonic::my_channel::{MyChannel, Change};
use hello_tonic::pb::greeter_client::GreeterClient;
use hello_tonic::pb::HelloRequest;
use tokio::runtime::Runtime;
use tonic::metadata::MetadataValue;



fn main(){
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let rt = Arc::new(rt);
    rt.clone().block_on(connect(rt)).unwrap();

}

async fn connect(rt: Arc<Runtime>)-> Result<(), Box<dyn std::error::Error>> {
    let channel  = MyChannel::new(rt.clone());
    let tx = channel.get_change_tx();

    let uris = ["http://localhost:50051", "http://localhost:50052"];
    rt.spawn(async move {
        uris.iter().for_each(|uri| {
            tx.send(Change::Insert(uri.to_string(), uri.to_string())).unwrap();
        });
        // 模拟10秒后移除第一个
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        tx.send(Change::Remove(uris[0].to_string())).unwrap();
    });


    // 使用超时的中间件
    // let channel = tower::timeout::Timeout::new(channel, Duration::from_millis(3000));
    // 普通的客户端
    // let mut client = GreeterClient::new(channel);

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
