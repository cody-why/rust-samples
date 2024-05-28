/***
 * @Author: plucky
 * @Date: 2022-09-03 17:57:20
 * @LastEditTime: 2022-09-15 15:22:55
 * @Description: 
 */

use std::net::SocketAddr;
use std::sync::atomic::AtomicU64;

use smol::{io::{self, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, stream::StreamExt};
use tracing::{info, debug};

use crate::config::Server;

static USEPASS:bool = false;
static STATUS:AtomicU64 = AtomicU64::new(0);

pub async fn start_server(config: &Server) -> Result<(), Box<dyn std::error::Error>> {
    // let listener = TcpListener::bind(&config.listen).await?;
    let listener = TcpListener::bind(&config.listen).await?;
    let mut incoming = listener.incoming();

    info!("Listening on: {}", config.listen);
    let white_list = config.white_list.clone();

    while let Some(stream) = incoming.next().await {
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        if !white_list.is_empty() && !white_list.contains(&addr.ip().to_string()) {
            info!("{} is not in white list", addr);
            continue; 
        }
        smol::spawn(async move {
            // stream.set_nodelay(false).ok();
            let e= handle(stream, addr).await;
            if let Err(e) = e {
                info!("{} close error: {}", addr, e);
            }
        }).detach();
    }


    Ok(())
}



// connect 协议
// https://jiajunhuang.com/articles/2019_06_06-socks5.md.html
async fn do_greeting(src_reader: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: Vec<u8> = vec![0x00; 256];
    // 读取2个字节
    src_reader.read_exact(&mut buf[0..2]).await?;
    debug!("do_greeting: {:?}", &buf[0..2]);
    // 判断是否是socks5协议的版本号
    if buf[0] != 0x05 {
        return Err("greeting unreachable!".into());
    }

    //认证方法的数量,也是数据的长度,比如0x02,表示有2种认证方法,数据是0x00,0x01
    let nauth = buf[1] as usize;
    //认证方法: 0x00 无认证,0x02 用户名密码认证
    src_reader.read_exact(&mut buf[0..nauth]).await?;

    // info!("nauth: {}", nauth);

    //不需要认证：0x05 0x00
    //用户名密码认证：0x05 0x02
    if !USEPASS {
        src_reader.write_all(&[0x05, 0x00]).await.expect("write error");
        return Ok(())
    }
    // 用户名密码认证
    src_reader.write_all(&[0x05, 0x02]).await?;

    //鉴定协议版本 0x1
    src_reader.read_exact(&mut buf[0..1]).await?;
    // info!("鉴定协议版本: {}", buf[0]);
    if buf[0] != 0x01 {
        return Err("nauth unreachable!".into());
    }
    src_reader.read_exact(&mut buf[0..1]).await?;
    let l = buf[0] as usize;
    src_reader.read_exact(&mut buf[0..l]).await?;
    let username = String::from_utf8_lossy(&buf[0..l]).to_string();
    
    src_reader.read_exact(&mut buf[0..1]).await?;
    let l = buf[0] as usize;
    src_reader.read_exact(&mut buf[0..l]).await?;
    let password = String::from_utf8_lossy(&buf[0..l]).to_string();

    // info!("username: {}, password: {}", username, password);

    // 0x05 0x00 认证成功
    if username == "user" && password == "pass*" {
        src_reader.write_all(&[0x05,0x00]).await?;
    }else{
        src_reader.write_all(&[0x05,0x01]).await?;
    }

    Ok(())
}

// 握手协议,解析客户端的请求
async fn parse_dst(src_reader: &mut TcpStream) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf: Vec<u8> = vec![0x00; 256];
    src_reader.read_exact(&mut buf[0..4]).await?;
    // 5,1,0,3
    // info!("buf: {:?}", &buf[0..4]);
    
    // 判断是否是socks5协议的版本号
    if buf[0] != 0x05 {
        return Err("parse_dst 1 unreachable!".into());
    }  
    if buf[1] != 0x01 {
        // 不支持0x01以外的SOCK命令码，0x01表示CONNECT请求
        return Err("parse_dst 2 unreachable!".into());
    }
    if buf[2] != 0x00 {
        return Err("parse_dst 3 unreachable!".into());
    }

    

    // let mut rbuf: Vec<u8> = vec![0x00; 256];
    // rbuf[0] = 0x05;
    // #[allow(unused_assignments)]
    // let mut n = 0;
    // 匹配ATYP BND.ADDR类型
    let host = match buf[3] {
        0x01 => {
            // IPv4地址，DST.ADDR部分4字节长度
            src_reader.read_exact(&mut buf[0..4]).await?;
            // rbuf[3] = 0x01;
            // rbuf[4..8].copy_from_slice(&buf[0..4]);
            // n = 8;
            std::net::Ipv4Addr::new(buf[0], buf[1], buf[2], buf[3]).to_string()
        }
        0x03 => {
            // 域名，DST.ADDR部分第一个字节为域名长度，DST.ADDR剩余的内容为域名，没有\0结尾
            src_reader.read_exact(&mut buf[0..1]).await?;
            let l = buf[0] as usize;
            src_reader.read_exact(&mut buf[0..l]).await?;
            // rbuf[3] = 0x03;
            // rbuf[4] = l as u8;
            // rbuf[5..(5+l)].copy_from_slice(&buf[0..l]);
            // n = 5+l;
            String::from_utf8_lossy(&buf[0..l]).to_string()  // example: baidu.com
        }
        0x04 => {
            // IPv6地址，16个字节长度
            src_reader.read_exact(&mut buf[0..16]).await?;
            // rbuf[3] = 0x04;
            // rbuf[4..20].copy_from_slice(&buf[0..16]);
            // n = 20;
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
        _ => return  Err("parse_dst host unreachable!".into()),
    };

    src_reader.read_exact(&mut buf[0..2]).await?;
    let port = ((buf[0] as u16) << 8) | (buf[1] as u16);
    let dst = format!("{}:{}", host, port);
    
    // rbuf[n] = buf[0];
    // rbuf[n+1] = buf[1];

    // src_reader.write_all(&rbuf[0..n+2]).await?;
    src_reader.write_all(&[0x05,0,0,0x01,0,0,0,0,0,0]).await?;
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
    Ok(dst)
}


// socks5协议维基百科：https://zh.m.wikipedia.org/zh-hans/SOCKS
// 英文：https://en.wikipedia.org/wiki/SOCKS#SOCKS5
async fn handle(mut src_stream: TcpStream, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    // let addr = src_stream.peer_addr()?;
    info!("{} ->", addr);
    do_greeting(&mut src_stream).await.expect("do_greeting error");
    // info!("greeting done");
    let dst = parse_dst(&mut src_stream).await.expect("parse_dst error");
    src_stream.flush().await.expect("flush error");

    info!("{} -> {}", addr, dst);


    let dst_stream = TcpStream::connect(&dst).await.expect("connect error");
    STATUS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    info!("{} -> {} connect {}", addr, dst, STATUS.load(std::sync::atomic::Ordering::Relaxed));

    // let  (mut src_reader,mut src_writer) = src_stream.r();
    // let  (mut dst_reader,mut dst_writer) = dst_stream.into_split();
    let mut src_reader = src_stream.clone();
    let mut dst_reader = dst_stream.clone();
    let mut src_writer = src_stream;
    let mut dst_writer = dst_stream;

    // let mut src_reader =  BufReader::new(src_reader);
    // let mut dst_reader =  BufReader::new(dst_reader);
    // let mut src_writer = BufWriter::new(src_writer);
    // let mut dst_writer = BufWriter::new(dst_writer);
    let ex = smol::Executor::new();
    ex.spawn(async move {
        // let _= io::copy(&mut src_reader, &mut dst_writer).await.map_err(|e| {
        //     info!("c_to_s error: {}", e);
        // });
        
        loop {
            let mut buf: Vec<u8> = vec![0x00; 4096];
            let n = src_reader.read(&mut buf).await.map_err(|e| {
                info!("c_to_s error: {}", e);
            });
            if n.is_err() {
                break;
            }
            let n = n.unwrap();
            if n == 0 {
                break;
            }
            if let Err(e) = dst_writer.write_all(&buf[0..n]).await{
                info!("c_to_s error: {}", e);
                break;
            };
            if let Err(e) = dst_writer.flush().await{
                info!("c_to_s error: {}", e);
                break;
            };
        }
        src_reader.close().await.ok();
        dst_writer.close().await.ok();
        
        
    }).detach();

    
    ex.run(async move {
        let _= io::copy(&mut dst_reader, &mut src_writer).await.map_err(|e| {
            info!("s_to_c error: {}", e);
        });
    }).await;
    
    
    // future::join(t1, t2).await;
    STATUS.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);

    info!("{} -> {} done {}",addr, dst, STATUS.load(std::sync::atomic::Ordering::Relaxed));
    Ok(())
}

