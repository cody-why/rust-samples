/*
 * @Author: plucky
 * @Date: 2023-01-09 12:02:40
 * @LastEditTime: 2023-01-09 22:58:52
 * @Description: 
 */

use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_listen: String,
    pub log_level: String,
    #[serde(default)]
    pub white_list: String,
}

impl Config {
    pub fn load(file:&str) -> Self {
        let path = std::env::current_exe().unwrap().parent().unwrap().join("");
        println!("{:?}", path);
        // rust if not debug
        #[cfg(not(debug_assertions))]
        std::env::set_current_dir(path).unwrap();
        
        serde_yaml::from_reader::<_, Config>(std::fs::File::open(file).unwrap()).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self{
            server_listen: "0.0.0.0:3080".into(),
            log_level: "info".into(),
            white_list: "".into(),
        }
    }
}


