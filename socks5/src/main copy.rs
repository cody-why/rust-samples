
// use std::io::{Read, Write};
// use tracing::info;

// const USER:&str = "user";
// const PASS:&str = "pasa*";

// static USERATUH:bool = false;

// fn main111() {
//     // std::env::set_var("RUST_LOG", "debug");
//     tracing_subscriber::fmt::init();

//     let mut c_listen = String::from("0.0.0.0:3080");
//     {
//         let mut ap = argparse::ArgumentParser::new();
//         ap.set_description("Socks5 Proxy");
//         // 定义支持的启动参数
//         ap.refer(&mut c_listen).add_option(
//             &["-l", "--listen"], argparse::Store, "listen address",
//         );
//         ap.parse_args_or_exit();
//     }

//     info!("Listen and server on {}", c_listen);

//     // 开启tcp监听器
//     let listener = std::net::TcpListener::bind(c_listen.as_str()).unwrap();
//     for stream in listener.incoming() {
//         match stream {
//             Ok(data) => {
//                 // 创建新的线程去处理请求
//                 std::thread::spawn(move || {
//                     if let Err(err) = handle(&data) {
//                         info!("error: {:?}", err)
//                     }
//                 });
//             }
//             Err(err) => {
//                 info!("error: {:?}", err);
//             }
//         }
//     }
// }

// // connect 协议
// // https://www.q578.com/s-5-2526694-0/
// fn do_greeting(src_reader: &mut std::net::TcpStream,
//                src_writer: &mut std::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
//     let mut buf: Vec<u8> = vec![0x00; 256];
//     // 读一个byte
//     src_reader.read_exact(&mut buf[0..2])?;
//     // 判断是否是socks5协议的版本号
//     if buf[0] != 0x05 {
//         panic!("greeting unreachable!");
//     }

    
//     //认证方法的数量,也是数据的长度,比如0x02,表示有2种认证方法,数据是0x00,0x01
//     let nauth = buf[1] as usize;
//     //认证方法: 0x00 无认证,0x02 用户名密码认证
//     src_reader.read_exact(&mut buf[0..nauth])?;

//     // info!("nauth: {}", nauth);

//     //服务器答复：
//     //不需要认证的数据为：0x05 0x00
//     //用户名密码认证：0x05 0x02
//     if !USERATUH {
//         src_writer.write(&[0x05,0x00])?;
//         info!("greeting done");
//        return Ok(())
//     }
    
//     // 用户名密码认证
//     src_writer.write(&[0x05,0x02])?;
//     //鉴定协议版本 0x1
//     src_reader.read_exact(&mut buf[0..1])?;
//     // info!("鉴定协议版本: {}", buf[0]);
//     if buf[0] != 0x01 {
//         panic!("nauth unreachable!");
//     }
//     src_reader.read_exact(&mut buf[0..1])?;
//     let len = buf[0] as usize;
//     src_reader.read_exact(&mut buf[0..len])?;
//     let username = String::from_utf8_lossy(&buf[0..len]).to_string();
    
//     src_reader.read_exact(&mut buf[0..1])?;
//     let len = buf[0] as usize;
//     src_reader.read_exact(&mut buf[0..len])?;
//     let password = String::from_utf8_lossy(&buf[0..len]).to_string();

//     // info!("username: {}, password: {}", username, password);

//     //服务器答复：0x05 0x00 认证成功
//     if username == USER && password == PASS {
//         src_writer.write(&[0x05,0x00])?;
//     }else{
//         src_writer.write(&[0x05,0x01])?;
//     }

//     info!("greeting done");

//     Ok(())
// }

// // 握手协议,解析客户端的请求
// fn parse_dst(src_reader: &mut std::net::TcpStream) -> Result<String, Box<dyn std::error::Error>> {
//     let mut buf: Vec<u8> = vec![0x00; 256];
//     src_reader.read_exact(&mut buf[0..4])?;
//     // 判断是否是socks5协议的版本号
//     if buf[0] != 0x05 {
//         panic!("parse_dst unreachable!");
//     }
//     if buf[1] != 0x01 {
//         // 不支持0x01以外的SOCK命令码，0x01表示CONNECT请求
//         panic!("unreachable!");
//     }
//     if buf[2] != 0x00 {
//         panic!("unreachable!");
//     }

//     // 匹配ATYP BND.ADDR类型
//     let host = match buf[3] {
//         0x01 => {
//             // IPv4地址，DST.ADDR部分4字节长度
//             src_reader.read_exact(&mut buf[0..4])?;
//             std::net::Ipv4Addr::new(buf[0], buf[1], buf[2], buf[3]).to_string()
//         }
//         0x03 => {
//             // 域名，DST.ADDR部分第一个字节为域名长度，DST.ADDR剩余的内容为域名，没有\0结尾
//             src_reader.read_exact(&mut buf[0..1])?;
//             let l = buf[0] as usize;
//             src_reader.read_exact(&mut buf[0..l])?;
//             String::from_utf8_lossy(&buf[0..l]).to_string()  // example: baidu.com
//         }
//         0x04 => {
//             // IPv6地址，16个字节长度
//             src_reader.read_exact(&mut buf[0..16])?;
//             std::net::Ipv6Addr::new(
//                 ((buf[0x00] as u16) << 8) | (buf[0x01] as u16),
//                 ((buf[0x02] as u16) << 8) | (buf[0x03] as u16),
//                 ((buf[0x04] as u16) << 8) | (buf[0x05] as u16),
//                 ((buf[0x06] as u16) << 8) | (buf[0x07] as u16),
//                 ((buf[0x08] as u16) << 8) | (buf[0x09] as u16),
//                 ((buf[0x0a] as u16) << 8) | (buf[0x0b] as u16),
//                 ((buf[0x0c] as u16) << 8) | (buf[0x0d] as u16),
//                 ((buf[0x0e] as u16) << 8) | (buf[0x0f] as u16),
//             ).to_string()
//         }
//         _ => panic!("unreachable!")
//     };

//     src_reader.read_exact(&mut buf[0..2])?;
//     let port = ((buf[0] as u16) << 8) | (buf[1] as u16);
//     let dst = format!("{}:{}", host, port);

//     Ok(dst)
// }

// // socks5协议维基百科：https://zh.m.wikipedia.org/zh-hans/SOCKS
// // 英文：https://en.wikipedia.org/wiki/SOCKS#SOCKS5
// fn handle(src_stream: &std::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
//     info!("src: {}", src_stream.peer_addr().unwrap());
//     let mut src_reader = src_stream.try_clone()?;
//     let mut src_writer = src_stream.try_clone()?;
//     do_greeting(&mut src_reader, &mut src_writer)?;
//     let dst = parse_dst(&mut src_reader)?;
//     info!("dst: {}", dst);

//     // 连接目标地址，转发请求并返回响应
//     let dst_stream = std::net::TcpStream::connect(&dst)?;
//     let mut dst_reader = dst_stream.try_clone()?;
//     let mut dst_writer = dst_stream.try_clone()?;
//     src_writer.write(&[0x05,0,0,0x01,0,0,0,0,0,0])?;
//     // 参考socks5对响应包的描述
//     // VER
//     src_writer.write(&[0x05])?;
//     // STATUS
//     src_writer.write(&[0x00])?;
//     // RSV
//     src_writer.write(&[0x00])?;
//     // BNDADDR
//     src_writer.write(&[0x01])?;
//     src_writer.write(&[0x00])?;
//     src_writer.write(&[0x00])?;
//     src_writer.write(&[0x00])?;
//     src_writer.write(&[0x00])?;
//     // BNDPORT
//     src_writer.write(&[0x00])?;
//     src_writer.write(&[0x00])?;

//     std::thread::spawn(move || {
//         std::io::copy(&mut src_reader, &mut dst_writer).ok();
//     });
//     std::io::copy(&mut dst_reader, &mut src_writer).ok();

//     Ok(())
// }

