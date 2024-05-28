/*
 * @Author: plucky
 * @Date: 2023-02-17 23:42:25
 * @LastEditTime: 2023-11-25 22:10:32
 * @Description: 
 */

use std::str::FromStr;

use time::{UtcOffset, format_description};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt::{time::OffsetTime, self}, filter, EnvFilter};
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, 
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct LogConfig{
    pub level: String,
    pub tofile: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            tofile: false,
        }
    }
}

pub fn setup_logging(config: &LogConfig) {
    // 设置输出时间为utc+8:00
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    // OffsetTime::local_rfc_3339().expect("Could not get local offset!");
    // 日志级别 my_crate=info
    let env_filter = EnvFilter::new(&config.level);
    
    let _filter = filter::Targets::new()
    .with_target("my_crate", LevelFilter::from_str(&config.level).unwrap());

    // 输出到控制台中
    let stdout_layer = fmt::layer().with_timer(local_time.clone())
    .pretty().with_writer(std::io::stdout);
    
    
    if config.tofile{
        // 输出到文件中
        let file_appender = tracing_appender::rolling::daily("logs", "app.log");
        // let file_appender=file_appender.with_filter(|meta| {
        //     meta.target() == "my_crate"
        // }).with_max_level(tracing::Level::INFO).and(std::io::stdout);
        // 在另外的线程中写入文件
        // let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        // Box::leak(Box::new(guard));

        let file_layer = fmt::layer().with_timer(local_time)
        .with_line_number(true)
        .with_ansi(false)
        .with_file(true)
        .with_writer(file_appender);
    
        // 注册
        tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .with(env_filter)
        .init();
    
        return ;
    } 
    
     // 注册
     tracing_subscriber::registry()
     .with(stdout_layer)
     .with(env_filter)
     .init();

   
    
   
}


pub fn setup_logging2(config: &LogConfig) {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::from_str(&config.level).unwrap())
    .with_target(false)
    .with_timer(local_time)
    .with_line_number(true)
    .with_file(true)
    .init();
}