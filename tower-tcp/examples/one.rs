// use std::collections::HashMap;
// use std::sync::Arc;
// use std::net::SocketAddr;
// use tokio::sync::mpsc;
// use tokio_stream::wrappers::ReceiverStream;
// use tower::{Service, ServiceExt, BoxError, util::BoxService};
// use bytes::Bytes;
//  // Define a type to represent a message router
// #[derive(Clone)]
// struct Router<T> {
//     routes: Arc<HashMap<u16, BoxService<Bytes, Bytes, BoxError>>>,
//     _marker: std::marker::PhantomData<T>,
// }
//  impl<T> Router<T> {
//     fn new() -> Self {
//         Self {
//             routes: Arc::new(HashMap::new()),
//             _marker: std::marker::PhantomData,
//         }
//     }
//      // Define a function to add a route to the router
//     fn route<S>(mut self, id: u16, service: S) -> Self
//     where
//         S: Service<Bytes, Response=Bytes, Error=BoxError> + Send + Sync + 'static + Iterator<Item=Bytes>,
//         S::Future: Send + 'static,
//         T: Send + Sync + 'static,
//     {
//         let boxed = service
//             .map(Box::new)
//             .map_err(|e| Box::new(e) as BoxError)
//             .and_then(|service| service.call);
//         self.routes.insert(id, BoxService::new(boxed));
//         self
//     }
//      // Define a function to process incoming messages
//     async fn process(&self, id: u16, data: Bytes) -> Result<Bytes, BoxError> {
//         if let Some(service) = self.routes.get(&id) {
//             let resp = service.clone().oneshot(data).await?;
//             Ok(resp)
//         } else {
//             Err(Box::new(format!("no handler for id {}", id)))
//         }
//     }
// }
//  // Define a function to handle incoming connections
// async fn handle_connection(
//     stream: tokio::net::TcpStream,
//     router: Router<()>,
// ) -> Result<(), BoxError> {
//     // Wrap the incoming stream in a `Service` and a `Stream`
//     let (tx, rx) = mpsc::unbounded_channel();
//     let (read_half, write_half) = stream.into_split();
//     let mut service = ReceiverStream::new(rx)
//         .map(|data| Ok(data))
//         .forward(write_half)
//         .map(|result| {
//             if let Err(e) = result {
//                 eprintln!("error: {:?}", e);
//             }
//         });
//     tokio::spawn(async move {
//         if let Err(e) = service.await {
//             eprintln!("connection error: {:?}", e);
//         }
//     });
//      // Loop over incoming messages and process them using the router
//     let mut buf = [0u8; 1024];
//     let mut read_half = tokio::io::BufReader::new(read_half);
//     loop {
//         let len = match read_half.read(&mut buf).await {
//             Ok(len) if len == 0 => break,
//             Ok(len) => len,
//             Err(e) => {
//                 eprintln!("read error: {:?}", e);
//                 break;
//             }
//         };
//         let data = Bytes::copy_from_slice(&buf[..len]);
//         let id = u16::from_be_bytes([buf[0], buf[1]]);
//         let resp = router.process(id, data).await.unwrap_or_else(|e| {
//             eprintln!("error processing message: {:?}", e);
//             Bytes::new()
//         });
//         tx.send(resp).unwrap_or_else(|e| {
//             eprintln!("error sending message: {:?}", e);
//         });
//     }
//      Ok(())
// }
//  #[tokio::main]
// async fn main() -> Result<(), BoxError> {
//     let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
//     let listener = tokio::net::TcpListener::bind(&addr).await?;
//     let router = Router::new()
//         .route(1, tower::service_fn(|req: Bytes| async move {
//             println!("received message with id 1: {:?}", req);
//             Bytes::from_static(b"hello from handler 1")
//         }))
//         .route(2, tower::service_fn(|req: Bytes| async move {
//             println!("received message with id 2: {:?}", req);
//             Bytes::from_static(b"hello from handler 2")
//         }));
//      loop {
//         let (stream, _) = listener.accept().await?;
//         let router = router.clone();
//         tokio::spawn(async move {
//             handle_connection(stream, router).await.unwrap_or_else(|e| {
//                 eprintln!("connection error: {:?}", e);
//             });
//         });
//     }
//      Ok(())
// }


fn main() {
    println!("Hello, world!");

}
