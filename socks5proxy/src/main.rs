/***
 * @Author: plucky
 * @Date: 2022-08-26 22:30:52
 * @LastEditTime: 2022-09-06 16:08:31
 * @Description: 
 */

use std::{error::Error};

mod socket;
use socket::*;
use tracing_subscriber::fmt::time::OffsetTime;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // std::env::set_var("RUST_LOG", "debug");
    let timer = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    tracing_subscriber::fmt().with_target(false).with_timer(timer).init();
    
    let mut addr = String::from("0.0.0.0:3080");
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("Socks5 Proxy");
        // 定义支持的启动参数
        ap.refer(&mut addr).add_option(
            &["-l", "--listen"], argparse::Store, "listen address",
        );
        ap.parse_args_or_exit();
    }

    start_server(addr).await?;
    Ok(())
}
