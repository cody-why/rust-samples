/*
 * @Author: plucky
 * @Date: 2023-03-08 21:47:39
 * @LastEditTime: 2023-03-08 22:22:28
 * @Description: 
 */




use log::{info, debug, LevelFilter};
use log_error::LogError;
use simplelog::{CombinedLogger,TermLogger, TerminalMode, ColorChoice,ConfigBuilder};
use time::UtcOffset;


fn main() {
    setup_logging().log_error("setup_logging");
    info!("Hello, world!");
    debug!("Hello, world!");
}


pub fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::new()
    .set_time_offset(UtcOffset::from_hms(8, 0, 0)?)
    .set_target_level(LevelFilter::Warn)
    .build();
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, config, TerminalMode::Mixed, ColorChoice::Auto),
            // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_bin.log").unwrap())
        ]
    )?;
    
    Ok(())
}