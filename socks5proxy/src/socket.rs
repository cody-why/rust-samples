/***
 * @Author: plucky
 * @Date: 2022-09-03 17:57:20
 * @LastEditTime: 2022-09-15 15:22:55
 * @Description: 
 */

use tokio::{net::{TcpStream, TcpListener }};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::info;

static USEPASS:bool = false;

pub async fn start_server(addr:String) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let _= handle(stream).await;
        });
    }

    Ok(())
}

// connect 协议
// https://www.q578.com/s-5-2526694-0/
async fn do_greeting(src_reader: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: Vec<u8> = vec![0x00; 256];
    // 读取2个字节
    src_reader.read_exact(&mut buf[0..2]).await?;
    // 判断是否是socks5协议的版本号
    if buf[0] != 0x05 {
        panic!("greeting unreachable!");
    }

    
    //认证方法的数量,也是数据的长度,比如0x02,表示有2种认证方法,数据是0x00,0x01
    let nauth = buf[1] as usize;
    //认证方法: 0x00 无认证,0x02 用户名密码认证
    src_reader.read_exact(&mut buf[0..nauth]).await?;

    // info!("nauth: {}", nauth);

    //不需要认证：0x05 0x00
    //用户名密码认证：0x05 0x02
    if !USEPASS {
        src_reader.write(&[0x05, 0x00]).await?;
        return Ok(())
    }
    // 用户名密码认证
    src_reader.write(&[0x05, 0x02]).await?;

    //鉴定协议版本 0x1
    src_reader.read_exact(&mut buf[0..1]).await?;
    // info!("鉴定协议版本: {}", buf[0]);
    if buf[0] != 0x01 {
        panic!("nauth unreachable!");
    }

    let l = src_reader.read_u8().await? as usize;
    src_reader.read_exact(&mut buf[0..l]).await?;
    let username = String::from_utf8_lossy(&buf[0..l]).to_string();
    
    let l = src_reader.read_u8().await? as usize;
    src_reader.read_exact(&mut buf[0..l]).await?;
    let password = String::from_utf8_lossy(&buf[0..l]).to_string();

    // info!("username: {}, password: {}", username, password);

    // 0x05 0x00 认证成功
    if username == "user" && password == "pasa*" {
        src_reader.write(&[0x05,0x00]).await?;
    }else{
        src_reader.write(&[0x05,0x01]).await?;
    }

    Ok(())
}

// 握手协议,解析客户端的请求
async fn parse_dst(src_reader: &mut TcpStream) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf: Vec<u8> = vec![0x00; 256];
    src_reader.read_exact(&mut buf[0..4]).await?;
    // 判断是否是socks5协议的版本号
    if buf[0] != 0x05 {
        panic!("parse_dst unreachable!");
    }  
    if buf[1] != 0x01 {
        // 不支持0x01以外的SOCK命令码，0x01表示CONNECT请求
        panic!("unreachable!");
    }
    if buf[2] != 0x00 {
        panic!("unreachable!");
    }

    // 匹配ATYP BND.ADDR类型
    let host = match buf[3] {
        0x01 => {
            // IPv4地址，DST.ADDR部分4字节长度
            src_reader.read_exact(&mut buf[0..4]).await?;
            std::net::Ipv4Addr::new(buf[0], buf[1], buf[2], buf[3]).to_string()
        }
        0x03 => {
            // 域名，DST.ADDR部分第一个字节为域名长度，DST.ADDR剩余的内容为域名，没有\0结尾
            src_reader.read_exact(&mut buf[0..1]).await?;
            let l = buf[0] as usize;
            src_reader.read_exact(&mut buf[0..l]).await?;
            String::from_utf8_lossy(&buf[0..l]).to_string()  // example: baidu.com
        }
        0x04 => {
            // IPv6地址，16个字节长度
            src_reader.read_exact(&mut buf[0..16]).await?;
            std::net::Ipv6Addr::new(
                ((buf[0x00] as u16) << 8) | (buf[0x01] as u16),
                ((buf[0x02] as u16) << 8) | (buf[0x03] as u16),
                ((buf[0x04] as u16) << 8) | (buf[0x05] as u16),
                ((buf[0x06] as u16) << 8) | (buf[0x07] as u16),
                ((buf[0x08] as u16) << 8) | (buf[0x09] as u16),
                ((buf[0x0a] as u16) << 8) | (buf[0x0b] as u16),
                ((buf[0x0c] as u16) << 8) | (buf[0x0d] as u16),
                ((buf[0x0e] as u16) << 8) | (buf[0x0f] as u16),
            ).to_string()
        }
        _ => panic!("unreachable!")
    };

    src_reader.read_exact(&mut buf[0..2]).await?;
    let port = ((buf[0] as u16) << 8) | (buf[1] as u16);
    let dst = format!("{}:{}", host, port);

    Ok(dst)
}


// socks5协议维基百科：https://zh.m.wikipedia.org/zh-hans/SOCKS
// 英文：https://en.wikipedia.org/wiki/SOCKS#SOCKS5
async fn handle(src_stream: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let addr = src_stream.peer_addr()?;
    // info!("src: {}", addr);
    
    let mut src_stream = src_stream;
    do_greeting(&mut src_stream).await?;
    // info!("greeting done");
    let dst = parse_dst(&mut src_stream).await?;
    info!("{} to {}", addr, dst);

    // 连接目标地址，转发请求并返回响应
    let dst_stream = TcpStream::connect(&dst).await?;

    src_stream.write_all(&[0x05,0,0,0x01,0,0,0,0,0,0]).await?;
    // 参考socks5对响应包的描述
    // VER STATUS RSV
    // src_writer.write(&[0x05])?;
    // STATUS
    // src_writer.write(&[0x00])?;
    // RSV
    // src_writer.write(&[0x00])?;
    // BNDADDR BNDPORT
    // src_writer.write(&[0x01])?;
    // src_writer.write(&[0x00])?;
    // src_writer.write(&[0x00])?;
    // src_writer.write(&[0x00])?;
    // src_writer.write(&[0x00])?;
    // BNDPORT
    // src_writer.write(&[0x00])?;
    // src_writer.write(&[0x00])?;

    let  (mut src_reader,mut src_writer) = src_stream.into_split();
    let  (mut dst_reader,mut dst_writer) = dst_stream.into_split();
   
    // _=tokio::join! (
    //     tokio::io::copy(&mut src_reader, &mut dst_writer),
    //     tokio::io::copy(&mut dst_reader, &mut src_writer),
    // );
    // tokio::select! {
    //     _=tokio::io::copy(&mut src_reader, &mut dst_writer) => {
    //         // info!("{} src copy dst done", dst);
    //     },
    //     _=tokio::io::copy(&mut dst_reader, &mut src_writer) => {
    //         // info!("{} dst copy src done", dst);
    //     }
    // }
    let dst2 = dst.clone();
    tokio::spawn(async move {
        tokio::io::copy(&mut src_reader, &mut dst_writer).await.unwrap_or(0);
        info!("{} src -> dst done", dst2);
    });
    tokio::io::copy(&mut dst_reader, &mut src_writer).await?;
    info!("{} done", dst);
    
    Ok(())
}

