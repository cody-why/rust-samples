/*
 * @Author: plucky
 * @Date: 2023-11-05 21:05:53
 * @LastEditTime: 2023-11-08 10:17:23
 */

pub mod my_channel;
pub mod etcd_register;
pub mod etcd_discovery;
pub mod greeter;
pub use my_channel::MyChannel;
pub use etcd_register::*;
pub use etcd_discovery::*;

pub mod pb {
    // tonic::include_proto!("grpc.echo");
    include!("./proto/echo.rs");
    include!("./proto/helloworld.rs");
}