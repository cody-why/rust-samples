/*
 * @Author: plucky
 * @Date: 2023-05-31 18:25:08
 * @LastEditTime: 2023-06-10 11:23:04
 * @Description: 
 */


// use std::{ future::Future, pin::Pin, sync::Arc};
// use bytes::{ Bytes };
// use std::collections::HashMap;


// #[derive(Clone)]
// pub struct Router {
//     handlers: HashMap<u16, Arc<dyn Fn(u16, Bytes) -> Pin<Box<dyn Future<Output = Bytes> + Send>> + Send + Sync>>,
// }
// impl Router {
//     pub fn new() -> Self {
//         Self {
//             handlers: HashMap::new(),
//         }
//     }
//     pub fn route<F, Fut>(&mut self, message_id: u16, handler: F) -> &mut Self
//         where
//             F: Fn(u16,Bytes) -> Fut + Send + Sync + 'static,
//             Fut: Future<Output = Bytes> + Send + 'static,
//     {
//         self.handlers.insert(message_id, Arc::new(move |id,msg| Box::pin(handler(id, msg))));
//         self
//     }
//     pub async fn dispatch(&self, message_id: u16, message: Bytes) -> Option<Bytes> {
//         if let Some(handler) = self.handlers.get(&message_id) {
//             let msg = handler(message_id, message).await;
//             Some(msg)
//         } else {
//             None
//         }
//     }
//     pub fn merge(&mut self, router: Router) {
//         self.handlers.extend(router.handlers);
        
//     }

// }


// #[cfg(test)]
// mod tests{
//     use super::*;
 

//     #[tokio::test]
//     async fn test_router(){
//         let state = 1;
//         let mut router = Router::new();

//         async fn handler(id:u16,msg: Bytes) -> Bytes {
//             println!("received message with id {id}: {:?}", msg);
            
//             msg.to_ascii_uppercase().into()
//         }

//         router.route(1, handler);
//         router.route(2, move |id,msg| async move {
//             println!("state {state}");
//             println!("received message with id {id}: {:?}", msg);
//             Bytes::from("hello world")
//         });

//         let msg = Bytes::from("hello world");
//         let res = router.dispatch(1, msg.clone()).await;
//         assert_eq!(res, Some(Bytes::from("HELLO WORLD")));

//         router.dispatch(2, msg).await;

        
//     }

// }


