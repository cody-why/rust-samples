/*
 * @Author: plucky
 * @Date: 2022-08-26 22:30:52
 * @LastEditTime: 2023-12-08 22:33:29
 * @Description: 
 */

use std::env::args;
use std::str::FromStr;

mod config;


use config::{Config, Server};
use tracing::metadata::LevelFilter;
use tracing_subscriber::fmt::time::OffsetTime;


mod socket;
use socket::*;
// mod socket_std;
// use socket_std::*;
// mod socket_smol;
// use socket_smol::*;
// mod socket_async_std;
// use socket_async_std::*;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async_main());
    std::env::set_var("SMOL_THREADS", "4");
    
    // let ex = smol::Executor::new();
    // smol::block_on(ex.run(async_main()));
    // async_std::task::block_on(async_main());
    
}

async fn async_main() {
    let file = args().nth(1).unwrap_or("config.yaml".into());
    let config = Config::load(&file);
    println!("{:?}", config);
    
    setup_logger(&config.server);
    
    start_server(&config.server).await.unwrap();
}




fn setup_logger(config: &Server){
    let timer = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    // 日志级别
    let lf = LevelFilter::from_str(&config.log_level).unwrap();
    tracing_subscriber::fmt()
        .with_target(false)
        .with_timer(timer)
        // .with_line_number(true)
        // .with_file(true)
        .with_max_level(lf).init(); 
}
