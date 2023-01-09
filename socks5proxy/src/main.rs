/*
 * @Author: plucky
 * @Date: 2022-08-26 22:30:52
 * @LastEditTime: 2023-01-09 22:58:22
 * @Description: 
 */

use std::env::args;
use std::str::FromStr;
use std::{error::Error};

mod socket;
mod config;
use config::Config;
use socket::*;
use tracing::metadata::LevelFilter;
use tracing_subscriber::fmt::time::OffsetTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // std::env::set_var("RUST_LOG", "debug");
    // 从arg获得参数
    let file = args().nth(1).unwrap_or("app.yaml".into());


    let config = Config::load(&file);
    println!("{:?}", config);
    
    
    let timer = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    // 日志级别
    let lf = LevelFilter::from_str(&config.log_level).unwrap();
    tracing_subscriber::fmt().with_target(false).with_timer(timer)
    .with_max_level(lf).init();
    
    
    
    start_server(&config).await?;
    Ok(())
}


// let mut addr = String::from("0.0.0.0:3080");
    // {
    //     let mut ap = argparse::ArgumentParser::new();
    //     ap.set_description("Socks5 Proxy");
    //     // 定义支持的启动参数
    //     ap.refer(&mut addr).add_option(
    //         &["-l", "--listen"], argparse::Store, "listen address",
    //     );
    //     ap.parse_args_or_exit();
    // }
