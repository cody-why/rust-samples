/*
 * @Author: anger
 * @Date: 2023-11-10 18:25:20
 * @LastEditTime: 2023-11-10 18:26:31
 */

use tonic::{Request, Response, Status};

use crate::pb::{greeter_server::Greeter, HelloReply, HelloRequest};


#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        tracing::info!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}
