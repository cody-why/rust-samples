/*
 * @Author: plucky
 * @Date: 2023-03-05 01:48:56
 * @LastEditTime: 2024-06-16 19:55:09
 * @Description: 
 */


use std::time::Duration;

use html_escape::decode_html_entities;
use tl::{self, ParserOptions};

use crate::lang::InputLang;



/// Translates the text.
///
/// # Examples
///
/// ```
/// translate("en","tr","cat");
/// ```
pub fn translate(from:&str, to:&str, text: &str) -> Result<String, String> {
    let _froml:InputLang = from.into();
    let _tol:InputLang = to.into();
    
    parse_result(fetch_page(text, from, to))
}

fn fetch_page(text: &str, from: &str, to: &str) -> Result<String, String> {
    let formatted_url = format!("https://translate.google.com/m?tl={}&sl={}&q={}&hl=en", to, from, text);
    println!("{}", formatted_url);
    // match reqwest::blocking::get(formatted_url) {
    let resp = reqwest::blocking::Client::new().get(formatted_url).timeout(Duration::from_secs(5)).send();
    
    match resp {    
        Ok(response) =>{
            match response.text() {
                Ok(body) => Ok(body),
                Err(err) => Err(err.to_string())
            }
        },
        Err(err) => Err(err.to_string())
    }
}

fn parse_result(result: Result<String, String>) -> Result<String, String> {
    match result {
        Ok(body) => {
            let dom = tl::parse(&body, ParserOptions::default()).unwrap();
            let parser = dom.parser();
            let mut elements = dom.get_elements_by_class_name("result-container");
            match elements.next() {
                Some(element) => {
                    let text = element.get(parser).unwrap().inner_text(parser);
                    // 转换html标记符为文本
                    return Ok(decode_html_entities(&text).into_owned());
                
                },
                
                None => Err(String::from("unexcepted error."))
            }
        }
        Err(err) => Err(err)
    }
}


#[test]
fn test1(){
    // https://translate.google.com/m?tl=en&sl=zh-Cn&q=真可惜&hl=en
    std::env::set_var("https_proxy", "socks5://18.217.7.158:1081");
    match translate("zh-Cn", "en", "真可惜") {
        Ok(translated) => println!("Result: {}", translated),
        Err(_) => println!("Something wrong...")
    }

}