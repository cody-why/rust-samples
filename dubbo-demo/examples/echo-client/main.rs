/*
 * @Author: plucky
 * @Date: 2022-10-28 18:02:53
 * @LastEditTime: 2022-10-28 22:13:54
 * @Description: 
 */
use dubbo::codegen::*;
use dubbo_demo::protos::echo::echo_client::EchoClient;
use dubbo_demo::protos::echo::EchoRequest;
use futures_util::StreamExt;

pub struct FakeFilter {}

// impl Filter for FakeFilter {
//     fn call(&mut self, req: Request<()>) -> Result<Request<()>, dubbo::status::Status> {
//         println!("fake filter: {:?}", req.metadata);
//         Ok(req)
//     }
// }

#[tokio::main]
async fn main() {
    let mut cli = EchoClient::new().with_uri("http://127.0.0.1:8889".to_string());
    // let mut unary_cli = cli.clone().with_filter(FakeFilter {});
    
    println!("-----------请求一个,回复一个-----------");
    let resp = cli
        .unary_echo(Request::new(EchoRequest {
            message: "message from client".to_string(),
        }))
        .await;
    let resp = match resp {
        Ok(resp) => resp,
        Err(err) => return println!("{:?}", err),
    };
    let (_parts, body) = resp.into_parts();
    println!("Response: {:?}", body);
    println!("");
    
    // 
    println!("-----------请求多个,回复一个-----------");
    let data = vec![
        EchoRequest {
            message: "msg1 from client streaming".to_string(),
        },
        EchoRequest {
            message: "msg2 from client streaming".to_string(),
        },
        EchoRequest {
            message: "msg3 from client streaming".to_string(),
        },
    ];
    let req = futures_util::stream::iter(data);
    let resp = cli.client_streaming_echo(req).await;
    let client_streaming_resp = match resp {
        Ok(resp) => resp,
        Err(err) => return println!("{:?}", err),
    };
    let (_parts, resp_body) = client_streaming_resp.into_parts();
    println!("client streaming, Response: {:?}", resp_body);
    println!("");

    // 
    println!("-----------请求多个,回复多个-----------");
    let data = vec![
        EchoRequest {
            message: "msg1 from client".to_string(),
        },
        EchoRequest {
            message: "msg2 from client".to_string(),
        },
        EchoRequest {
            message: "msg3 from client".to_string(),
        },
    ];
    let req = futures_util::stream::iter(data);

    let bidi_resp = cli.bidirectional_streaming_echo(req).await.unwrap();

    let (parts, mut body) = bidi_resp.into_parts();
    println!("parts: {:?}", parts);
    while let Some(item) = body.next().await {
        match item {
            Ok(v) => {
                println!("reply: {:?}", v);
            }
            Err(err) => {
                println!("err: {:?}", err);
            }
        }
    }
    let trailer = body.trailer().await.unwrap();
    println!("trailer: {:?}", trailer);
    println!("");

    // 
    println!("-----------请求一个,回复多个-----------");
    let resp = cli
        .server_streaming_echo(Request::new(EchoRequest {
            message: "server streaming req".to_string(),
        }))
        .await
        .unwrap();

    let (parts, mut body) = resp.into_parts();
    println!("parts: {:?}", parts);
    while let Some(item) = body.next().await {
        match item {
            Ok(v) => {
                println!("reply: {:?}", v);
            }
            Err(err) => {
                println!("err: {:?}", err);
            }
        }
    }
    let trailer = body.trailer().await.unwrap();
    println!("trailer: {:?}", trailer);
}