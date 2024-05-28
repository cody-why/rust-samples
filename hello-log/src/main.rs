/*
 * @Author: plucky
 * @Date: 2023-02-17 21:28:20
 * @LastEditTime: 2024-01-25 09:52:28
 * @Description: 
 */




fn main() {
    println!("Hello, world!");

}

    // env_logger::Builder::new().filter_level(log::LevelFilter::Info)
    // .format(|buf, record| {
    //     writeln!(
    //         buf,
    //         "{} [{}] {}: {}",
    //         fastdate::DateTime::now(),
    //         record.level(),
    //         record.target(),
    //         record.args()
    //     )
    // }).init();
    
    // simple_logger::SimpleLogger::new()
    //     .with_utc_offset(time::UtcOffset::from_hms(8, 0, 0).unwrap())
    //     .with_level(log::LevelFilter::Info)
    //     .init().unwrap();