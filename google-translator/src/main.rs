/*
 * @Date: 2023-03-02 17:12:50
 * @LastEditTime: 2024-06-16 21:23:11
 * @Description: cargo build --release --target x86_64-pc-windows-gnu
 */

use std::{fs::File, io::Write};

use clap::Parser;
use indexmap::IndexMap;
use serde::Deserialize;

pub mod lang;
pub mod multiple;
pub mod single;
mod translator;
use serde_json::Value;
pub use translator::*;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub sigle_file: bool,
    pub file: String,
    pub from: String,
    pub json_key: String,
    pub translate: Vec<Translate>,
    pub https_proxy: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Translate {
    pub to: String,
    pub json_key: String,
}

static DEFAULT_CONFIG:&str = include_str!("../i18n.config.json");
// sock5代理需要设置环境变量, reqwest打开socks feature
// export https_proxy=socks5://xxx.xx.xx.xx:8080
fn main() {
    init_env();
    let args = Args::parse();
    let file = "i18n.config.json";
    let file_name = args.config.unwrap_or(file.to_string());
    if args.init {
        
        let map: IndexMap<String, Value> = serde_json::from_str(DEFAULT_CONFIG).unwrap();
        let json = serde_json::to_string_pretty(&map).unwrap();
        File::create(file_name).unwrap().write_all(json.as_bytes()).unwrap();
        println!("Config file created: {:?}", file );
        return;

    }
    
    println!("Config file: {}", file_name);
    let config: Config = serde_json::from_reader(File::open(file_name).unwrap()).unwrap();
    println!("{:?}", config);

    if config.sigle_file {
        multiple::process(config);
    } else {
        single::process(config);
    }

    println!("Press enter key to continue...");
    // std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn init_env() {
    std::env::set_var("RUST_LOG", "info");
    // env_logger::init();
    // let path = std::env::current_exe().unwrap();
    // println!("{}", path.display());

    // // 如果是release模式,则切换到json文件所在目录
    // if cfg!(not(debug_assertions)) {
    //     let path = path.parent().unwrap();
    //     std::env::set_current_dir(path).unwrap();
    // }
}

#[allow(unused)]
fn get_path() -> std::path::PathBuf {
    let path = std::env::current_exe().unwrap();
    if cfg!(not(debug_assertions)) {
        path.parent().unwrap().to_path_buf()
    } else {
        ".".parse().unwrap()
    }

}

/// Simple translate text
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// init config file
    #[arg(short, long)]
    init: bool,

    /// Config file name (default: i18n.config.json)
    #[arg(short, long)]
    config: Option<String>,
}
