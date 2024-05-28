/*
 * @Author: plucky
 * @Date: 2023-04-20 17:21:45
 * @LastEditTime: 2023-04-22 17:30:44
 * @Description: 
 */
// 在main加这个写代码时忽略很多警告
#![allow(unused)]

use crate::prelude::*;

mod error;
mod prelude;
mod utils;

fn main() -> Result<()>{
    println!("Hello, world!");

    Ok(())
    // Err("My error".into())
}
