/*
 * @Author: plucky
 * @Date: 2022-10-10 11:51:28
 * @LastEditTime: 2022-10-10 16:29:02
 * @Description: 
 */

use std::process::Command;

fn main(){
    // println!("hello, build.rs");
    // tailwindcss -i index.css -o output.css
    Command::new("tailwindcss").args(&["-i", "index.css", "-o", "output.css"])
        .status().unwrap();
}