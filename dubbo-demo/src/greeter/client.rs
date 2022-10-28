/*
 * @Author: plucky
 * @Date: 2022-10-28 11:38:39
 * @LastEditTime: 2022-10-28 15:54:41
 * @Description: 
 */


use dubbo::invocation::Request;
use dubbo_demo::protos::greeter::{greeter_client::*, GreeterRequest};


#[tokio::main]
async fn main() {
    let mut cli = GreeterClient::new().with_uri("http://127.0.0.1:8888".to_string());
    println!("# unary call");
    let resp = cli
        .greet(Request::new(GreeterRequest {
            name: "message from client".to_string(),
        }))
        .await;
    let resp = match resp {
        Ok(resp) => resp,
        Err(err) => return println!("{:?}", err),
    };
    let (_parts, body) = resp.into_parts();
    println!("Response: {:?}", body);
}