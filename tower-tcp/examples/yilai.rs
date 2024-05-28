/*
 * @Author: plucky
 * @Date: 2023-05-14 21:56:23
 * @LastEditTime: 2023-05-14 22:04:23
 * @Description: 
 */

use std::sync::Arc;
use tower::Service;
use tower::BoxError;
use tower::ServiceExt;

 #[derive(Clone)]
struct MyService {
    db: Arc<String>,
}
 impl Service<String> for MyService {
    type Response = String;
    type Error = BoxError;
    type Future = futures::future::Ready<Result<String, BoxError>>;
     
    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    
     fn call(&mut self, req: String) -> Self::Future {
        let response = format!("{} - {}", self.db, req);
        futures::future::ready(Ok(response))
    }

    
}
 fn main() {
    let db = Arc::new("DB Connection".to_string());
    let my_service = MyService { db };
    
     // Create a tower service from the `MyService` instance
    let mut svc = tower::service_fn(move |req: String| {
        my_service.clone().call(req)
    });

     // Use the service
    let response = futures::executor::block_on(
        async {
            svc.ready().await?.call("Request".to_string()).await
        }
    );
    println!("{:?}", response.unwrap());
}