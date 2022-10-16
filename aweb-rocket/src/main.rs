/***
 * @Author: plucky
 * @Date: 2022-08-13 18:15:19
 * @LastEditTime: 2022-09-16 20:51:03
 * @Description: 
 */

#[macro_use] extern crate rocket;

use rocket::{Config, fs::{ FileServer}};
use tracing::{debug, info};
mod config;

#[rocket::launch]
 async fn run() -> _ {
    let config = config::load_config();

    config::init_log(&config.log);
    info!("{:#?}", config);

    debug!("debug is ok");

    let mut cf = Config::default();
    // .address("0.0.0.0")
    cf.port = config.server.port;
    
    rocket::custom(cf)
    .mount("/", rocket::routes![manual::second])
    .mount("/", FileServer::from("."))
   
}

// #[get("/<name>")]
// fn hello(name: String) -> String {
//     format!("Hello, {}!", name)
// }



mod manual {
    use std::path::{PathBuf, Path};
    use rocket::fs::NamedFile;
    // use rocket::fs::relative;

    #[get("/second/<path..>")]
    pub async fn second(path: PathBuf) -> Option<NamedFile> {
        let mut path = Path::new("static").join(path);
        if path.is_dir() {
            path.push("index.html");
        }

        NamedFile::open(path).await.ok()
    }
}

