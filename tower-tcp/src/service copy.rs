/*
 * @Author: plucky
 * @Date: 2023-05-07 18:41:46
 * @LastEditTime: 2023-05-07 18:42:00
 * @Description: 
 */


// use std::{ future, task::{ Context, Poll }, pin::Pin, rc::Rc, sync::Arc };
// use bytes::{ Bytes, Buf };
// use futures::Future;
// use tower::Service;
// use std::collections::HashMap;

// #[derive(Clone)]
// pub struct MyService<F>
//     where
//         F: Fn(Bytes) -> Bytes,
//     {
//         times: usize,
//         handlers: HashMap<u32, F>,
//     }
    
//     impl<F> MyService<F>
//     where
//         F: Fn(Bytes) -> Bytes ,
//     {
//         pub fn new() -> Self {
//             Self {
//                 times: 0,
//                 handlers: HashMap::new(),
//             }
//         }
    
//         pub fn add_handler(&mut self, message_id: u32, handler: F) {
//             self.handlers.insert(message_id, handler);
//         }
    
//         fn handle_message(&self, message_id: u32, message: Bytes) -> Option<Bytes> {
//             if let Some(handler) = self.handlers.get(&message_id) {
//                 Some(handler(message))
//             } else {
//                 None
//             }
//         }
//     }

// impl<F>  Service<Bytes> for MyService<F>
//     where
//         F: Fn(Bytes) -> Bytes + Clone + Send + Sync + 'static,
// {
//     type Response = Bytes;
//     type Error = Box<dyn std::error::Error + Send + Sync>;
//     // type Future = future::Ready<Result<Self::Response, Self::Error>>;
//     // 返回异步的结果
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send >>;

//     fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, req: Bytes) -> Self::Future {
//         self.times += 1;
//         println!("times: {}, req: {}", self.times, String::from_utf8_lossy(&req));
//         // future::ready(Ok(Bytes::new()))
        
//         let this = Arc::new(self.clone());
//         let fut = async move {
//             if req.len() < 4 {
//                 return Ok(Bytes::new());
//             }
//             let message_id = u32::from_be_bytes([req[0], req[1], req[2], req[3]]);
//             let message = req.slice(4..);
//             if let Some(response) = this.handle_message(message_id, message) {
//                 return Ok(response);
//             } 
//             Err("no handler".into())
//         };
        
//         Box::pin(fut)
//     }
// }
