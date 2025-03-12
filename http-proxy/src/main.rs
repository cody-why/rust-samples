/*
 * @Author: plucky
 * @Date: 2023-03-04 08:57:08
 * @LastEditTime: 2023-03-06 09:01:15
 * @Description: cargo build --release --target x86_64-pc-windows-gnu
 */

use std::env::args;

mod http_proxy;

//
// http_proxy 3082
#[tokio::main]
async fn main() {
    setup_logging().expect("Failed to setup logging");
    // info!(target: "hello", "Hello, world! 222");

    let port = args().nth(1).unwrap_or("3082".into());
    let addr = format!("{}:{}", "0.0.0.0", port);
    let fix_host = args().nth(2).unwrap_or("1".into()) == "1";
    http_proxy::run(&addr, fix_host).await.unwrap();
}

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}:{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(log::LevelFilter::Info)
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
        // Apply globally
        .apply()?;
    Ok(())
}
