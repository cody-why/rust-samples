/*
 * @Author: plucky
 * @Date: 2023-05-31 18:25:08
 * @LastEditTime: 2023-06-06 20:03:38
 * @Description: 
 */

// #[derive(Clone)]
// pub struct Router {
//     handlers: HashMap<u16, Arc<dyn Fn(Bytes) -> Bytes + Send + Sync >>,
// }

// impl Router {
//     pub fn new() -> Self {
//         Self {
//             handlers: HashMap::new(),
//         }
//     }
    
//     /// 添加路由
//     pub fn route<F>(&mut self, message_id: u16, handler: F) -> &mut Self
//     where
//         F: Fn(Bytes) -> Bytes + Send + Sync + 'static,
//     {
//         self.handlers.insert(message_id, Arc::new(handler));
//         self
//     }
    
//     /// 派发消息
//     pub fn dispatch(&self, message_id: u16, message: Bytes) -> Option<Bytes> {
//         if let Some(handler) = self.handlers.get(&message_id) {
//             Some(handler(message))
//         } else {
//             None
//         }
//     }

//     /// 合并路由
//     pub fn merge(&mut self, router: Router) {
//         self.handlers.extend(router.handlers);
//     }

// }

// #[cfg(test)]
// mod tests{
//     use bytes::Bytes;
//     use super::*;


//     #[test]
//     fn test_router(){
//         let mut router = Router::new();
//         router.route(1, |msg| msg);
//         router.route(2, |msg| msg);

//         let mut router2 = Router::new();
//         router2.route(3, |msg| msg);
//         router.merge(router2);

//         assert_eq!(router.dispatch(1, Bytes::from("hello")), Some(Bytes::from("hello")));


//     }
// }