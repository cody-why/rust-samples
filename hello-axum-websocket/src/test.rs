/*
 * @Author: plucky
 * @Date: 2022-09-02 20:19:32
 * @LastEditTime: 2022-11-01 12:09:39
 * @Description: 
 */

#[cfg(test)]
mod tests {
    // use crate::*;
    use futures::SinkExt;
    use futures::StreamExt;
    use tokio_tungstenite::{connect_async, tungstenite::Message};

    #[tokio::test]
    async fn test_websocket() {
        for i in 0..1{
            connect_websocket(i).await;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }

    async fn connect_websocket(user: usize) {
        
        let url = url::Url::parse(&format!("ws://localhost:8088/websocket/group1/user{}",user)).expect("Can't parse url");
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (mut write, mut read) = ws_stream.split();

        write.send(Message::Text(format!("test")))
            .await
            .expect("Failed to send message");
        // if let Some(Ok(message)) = read.next().await {
        //     assert_eq!(message, Message::Text(format!("user1: test")));
        // }
        tokio::spawn({
            async move {
                while let Some(Ok(message)) = read.next().await {
                    println!("{} message: {}", user,message);
                }
            }
        });
        
            
        
    }
}