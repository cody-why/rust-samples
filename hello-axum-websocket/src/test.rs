/***
 * @Author: plucky
 * @Date: 2022-09-02 20:19:32
 * @LastEditTime: 2022-09-02 20:26:06
 * @Description: 
 */

#[cfg(test)]
mod tests {
    // use crate::*;
    use futures::SinkExt;
    use futures::StreamExt;
    use tokio_tungstenite::{connect_async, tungstenite::Message};

    #[tokio::test]
    async fn connect_websocket() {
        
        let url =
            url::Url::parse("ws://localhost:8088/websocket/group1/user1").expect("Can't parse url");
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();

        write
            .send(Message::Text(format!("test")))
            .await
            .expect("Failed to send message");
        if let Some(Ok(message)) = read.next().await {
            assert_eq!(message, Message::Text(format!("user1: test")));
        }
    }
}