/*
 * @Author: plucky
 * @Date: 2023-08-22 09:47:19
 * @LastEditTime: 2023-08-29 18:58:10
 */
#![allow(dead_code)]

use std::sync::Arc;

use tower::Service;

fn main(){
    
}

struct Connection{
    pub sum: u32
}

struct MyServer{

}

struct MyService{
    conn: Arc<Connection>,
}

impl Service<String> for MyService {
    type Response = String;
    type Error=String;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: String) -> Self::Future {
        println!("Connection: {}", self.conn.sum);
        std::future::ready(Ok(req))
    }
}

async fn feature_ref(arg: &mut Connection) {
    arg.sum += 1;
    println!("{}", arg.sum);
}

// let make_service = service_fn(|_: usize| {
//     async move {
//         Ok::<_, std::convert::Infallible>(event_service)
//     }
// });
// call_make_service(make_service).await;
    
// #[allow(dead_code)]
// async fn call_make_service<S, R>(mut make_service: S)
// where
//     S: Service<usize, Response = R>,
//     R: Service<NetEvent>,
//     S::Error: std::fmt::Debug,
//     R::Error: std::fmt::Debug,
// {
//     let mut servic = make_service.call(0).await.unwrap();
//     let (sender, _) = tokio::sync::mpsc::channel::<Bytes>(10);
//     let c = Connection::new("".to_string(), sender);
//     let c = Arc::new(c);
//     let c = Box::leak(Box::new(c));
//     servic.call(NetEvent::Connect(c)).await.unwrap();
// }