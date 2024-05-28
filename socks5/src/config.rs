/*
 * @Author: plucky
 * @Date: 2023-01-09 12:02:40
 * @LastEditTime: 2023-08-07 09:50:24
 * @Description: 
 */

use std::fs;

use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub listen: String,
    pub log_level: String,
    #[serde(default)]
    pub white_list: Vec<String>,
}


impl Config {
    pub fn load(file:&str) -> Self {
        let path = std::env::current_exe().unwrap().parent().unwrap().join("");
        println!("{:?}", path);
        // rust if not debug
        #[cfg(not(debug_assertions))]
        std::env::set_current_dir(path).unwrap();
        
        fs::read_to_string(file).map(|s| serde_yaml::from_str(&s).unwrap()).unwrap()
    }
}

impl Default for Server {
    fn default() -> Self {
        Self{
            listen: "0.0.0.0:3080".into(),
            log_level: "info".into(),
            white_list: vec![],
        }
    }
}


