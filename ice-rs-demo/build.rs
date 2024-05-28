/*
 * @Author: plucky
 * @Date: 2022-11-17 22:28:03
 * @LastEditTime: 2023-11-05 08:14:49
 * @Description: 
 */
#![allow(unused)]
use ice_rs::slice::parser;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Contact.ice");
    
    let root_module = parser::parse_ice_files(&vec![String::from("./Contact.ice")], ".")?;
    // root_module.generate(Path::new("./src/gen"), "")
    Ok(())
}