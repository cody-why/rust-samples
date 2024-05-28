/*
 * @Author: plucky
 * @Date: 2023-09-22 18:55:30
 * @LastEditTime: 2023-09-22 19:17:44
 */

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::config;

pub fn process(file_name: &str) {
    let config = config::load_config(file_name);
    let proxys = config.https_proxys.iter();
    proxys.for_each(|(k, v)| {
        println!("{}: {}", k, v);
        std::env::set_var("https_proxy", v);
        // 请求网站,检测是否成功
        config.urls.par_iter().for_each(|url| {
            let ok = match fetch_page(url) {
                Ok(body) => {println!("{}: {}", url, body.len());
                    true
            },
                Err(err) => {println!("{}: {}", url, err); false}
            };

            if !ok {
                // 记录到日志
                println!("{}: failed", url);
            }
        });
        
    })
    
}


pub fn fetch_page(url: &str) -> Result<String, String> {
    println!("{}", url);
    match reqwest::blocking::get(url) {
        
        Ok(response) =>{
            match response.text() {
                Ok(body) => Ok(body),
                Err(err) => Err(err.to_string())
            }
        },
        Err(err) => Err(err.to_string())
    }
}
