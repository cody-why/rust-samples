/*
 * @Author: plucky
 * @Date: 2023-05-05 23:03:50
 * @LastEditTime: 2023-05-15 16:21:45
 * @Description: 
 */

#[cfg(test)]
mod tests{
    #![allow(unused_imports)]
    use std::time::Duration;

    use tokio::io::{AsyncWriteExt, AsyncReadExt};

    
    #[tokio::test]
    async fn test_client(){
        let addr = "127.0.0.1:8080";
        let mut client = tokio::net::TcpStream::connect(addr).await.unwrap();
        
        let msg = "hello world 你好";
        // 发送消息id
        client.write(&1_u16.to_be_bytes()).await.unwrap();
        client.write_all(msg.as_bytes()).await.unwrap();
        
        tokio::time::sleep(Duration::from_millis(100)).await;

// 发送消息id
client.write(&2_u16.to_be_bytes()).await.unwrap();
        let msg = "你好 hello world";
        client.write_all(msg.as_bytes()).await.unwrap();

        let mut buf = [0u8; 1024];
        let n = client.read(&mut buf).await.unwrap();
        println!("read: {} ", String::from_utf8_lossy(&buf[..n]));
      
        // tokio::time::sleep(Duration::from_millis(1000)).await;
        // assert_eq!(&buf[..n], msg.as_bytes().to_ascii_uppercase());
        
    }
}