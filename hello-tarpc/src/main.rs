/*
 * @Author: plucky
 * @Date: 2023-07-14 23:53:39
 * @LastEditTime: 2023-07-18 16:28:53
 */

// use futures::StreamExt;
// use tarpc::{context, server::{self, incoming::Incoming, Channel}, tokio_serde::formats::Json};
//

// #[tokio::main]
// async fn main()-> anyhow::Result<()> {
//     let server_addr = "127.0.0.1:3000";

//     let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
//     // tracing::info!("Listening on port {}", server_addr);
//     listener.config_mut().max_frame_length(usize::MAX);
//     listener
//         // Ignore accept errors.
//         .filter_map(|r| future::ready(r.ok()))
//         .map(server::BaseChannel::with_defaults)
//         // Limit channels to 1 per IP.
//         .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
//         // serve is generated by the service attribute. It takes as input any type implementing
//         // the generated World trait.
//         .map(|channel| {
//             let server = HelloServer(channel.transport().peer_addr().unwrap());
//             channel.execute(server.serve())
//         })
//         // Max 10 channels.
//         .buffer_unordered(10)
//         .for_each(|_| async {})
//         .await;

//     Ok(())
// }

mod service;


fn main(){}