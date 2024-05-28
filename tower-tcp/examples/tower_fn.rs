/*
 * @Author: plucky
 * @Date: 2023-05-11 20:21:16
 * @LastEditTime: 2023-05-12 21:57:14
 * @Description: 
 */

// tower features utils
#![allow(unused)]
// use hyper::{Request, Response};
use tower::{service_fn, Service, ServiceExt, BoxError};

#[derive(Debug)]
struct Request<T>{
    body: T,
}
impl<T> Request<T>{
    fn new(body: T) -> Self {
        Self {
            body,
        }
    }
}

#[derive(Debug)]
struct Response<T>{
    body: T,
}
impl<T> Response<T>{
    fn new(body: T) -> Self {
        Self {
            body,
        }
    }
    fn into_body(self) -> T {
        self.body
    }
}

async fn handle(request: Request<&str>) -> Result<Response<&str>, BoxError> {
    println!("request: {:?}", request);
    let response = Response::new("Hello, World!");
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let mut service = service_fn(handle);

    let response = service.ready().await?
        .call(Request::new("Hello, World!")).await?;

    println!("response: {:?}", response);
    

    let mut service = service_fn(|request: String| async {
        let response = Response::new("Hello, World!");
        Ok::<_, BoxError>(response)
    });
    
    let response = service.ready().await?.call("Request".into()).await?;
    println!("response: {:?}", response);
    
    Ok(())
}