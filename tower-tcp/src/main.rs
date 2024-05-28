/*
 * @Author: plucky
 * @Date: 2023-05-05 22:14:32
 * @LastEditTime: 2023-06-08 20:23:58
 * @Description: 
 */

// /*
//  * @Author: plucky
//  * @Date: 2023-05-05 22:14:32
//  * @LastEditTime: 2023-06-06 20:39:10
//  * @Description:
//  */

// #![allow(dead_code)]

// mod test_client;
// mod service;
// mod message;
// mod server;
// mod router;
// mod service_fn;
// mod request;

// use std::{ println, time::Duration };
// use bytes::{Bytes};


// use crate::service::{ Router};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut router = Router::new();
//     let  num = 1;

//     async fn sum_fun(msg: Bytes) -> Bytes {
//         msg.to_ascii_uppercase().into()
//     }

//     let _sum = sum_fun;


//     router.route(1, move |msg|{
//         println!("received message with id 1: {:?}", msg);
//         println!("num: {}", num);
//         std::thread::sleep(Duration::from_millis(10000));
//         msg.to_ascii_uppercase().into()

//     });

//     router.route(2, |msg|{
//         println!("received message with id 2: {:?}", msg);
//         std::thread::sleep(Duration::from_millis(10000));
//         Bytes::from("hello world")
//     });
    

//     let addr =  "127.0.0.1:8080";
//     server::Server::bind(addr).serve(router).await?;
//     Ok(())

// }

fn main() {
    println!("Hello, world!");
}

mod service_fn;
mod request;
mod router;