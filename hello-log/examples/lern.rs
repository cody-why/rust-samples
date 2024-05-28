/*
 * @Author: plucky
 * @Date: 2023-03-06 08:54:56
 * @LastEditTime: 2023-11-25 23:09:48
 * @Description: 
 */
#![allow(unused, unused_macros)]
use std::time::{SystemTime, Duration};

macro_rules! info {
    ($($arg:tt)+) => {
        log::info!(target: "proxy", $($arg)+)
    };
}

macro_rules! debug {
    ($($arg:tt)+) => {
        log::debug!(target: "proxy", $($arg)+)
    };
}

fn main() {
    setup_logging().unwrap();
    log::info!("Hello, world!");
    log::debug!("Hello, world!");
    // work
    tracing::debug!("Hello, world! a");
}


pub fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
    .format(|out, message, record| {
        out.finish(format_args!(
            "{} {} {}:{} {}",
            // chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            humantime::format_rfc3339_millis(SystemTime::now().checked_add(Duration::from_secs(8 * 3600)).unwrap()),
            record.level(),
            record.target(),
            // record.file().unwrap_or(""),
            record.line().unwrap_or(0),
            message
        ))
    })
    // .level(log::LevelFilter::Debug)
    .level_for("hyper", log::LevelFilter::Info)
    // Output to stdout, files, and other Dispatch configurations
    .chain(std::io::stdout())
    // .chain(fern::Dispatch::new()
    //     .filter(|metadata| {
    //         metadata.target().starts_with("hello")
    //     })
    //     .chain(fern::DateBased::new("log/", "%Y-%m-%d.app.log"))
    // )
    // .chain(fern::log_file("log/debug.log")?)
    .apply()?;
   Ok(())
}

fn setup_logging2() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(fern::DateBased::new("program.log.", "%Y-%m-%d"))
        .apply()?;

    Ok(())
}