/***
 * @Author: plucky
 * @Date: 2022-08-28 22:38:42
 * @LastEditTime: 2022-08-29 16:49:23
 * @Description: http客户端 reqwest
 */
// #![allow(unused_imports)]
#![allow(dead_code)]
use std::{error::Error, time::Duration, collections::HashMap};

use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    demo1().await?;
    
    Ok(())
}

// 基本请求
async fn demo1()-> reqwest::Result<()>{
    let b = reqwest::get("https://swapi.dev/api/people")
    // let b  = reqwest::get("https://httpbin.org/ip")
    .await?
    .json::<HashMap<String,String>>()
    // .text()
    .await?;

    println!("Got {:?}", b);
    Ok(())
}


// Client提供诸如 get, post, put, delete ... 之类的方法，以及 request(&self, method: Method, url: U)
async fn demo2() -> reqwest::Result<()>{
    let client = reqwest::Client::new();
    let doge = client
        .get("https://api.coinstats.app/public/v1/coins/dogecoin")
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
    println!("{:}", doge);
    Ok(())
}


#[derive(Deserialize, Debug)]
struct Response {
    coins: Vec<Coin>,
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Coin {
    id: String,
    name: String,
    icon: String,
    symbol: String,
    price: f32,
    priceBtc: f32,
}


async fn demo3() -> reqwest::Result<()>{
    let http_response = reqwest::get("https://api.coinstats.app/public/v1/coins?skip=0&limit=10").await?;
    let response = http_response.json::<Response>().await?;
    println!("{:#?}", response.coins);
    Ok(())
}