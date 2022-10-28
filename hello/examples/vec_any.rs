/*
 * @Author: plucky
 * @Date: 2022-10-19 16:41:21
 * @LastEditTime: 2022-10-19 16:41:42
 * @Description: 
 */
use std::any::Any;


fn main(){
    let vec:Vec<Box<dyn Any>> = vec![Box::new(1), Box::new("plucky"),Box::new("slsl".to_string())];//:Vec<Box<dyn Display>> 
    for v in vec {
        // if type is string
        
        if v.is::<i32>(){
            println!("is i32 {:?}",v);
        }
        if let Some(string) = v.downcast_ref::<String>() {
            println!("It's a string: '{}'", string);
        } else {
            println!("Not a string... {:?}",v);
        }
    }
    // std::vec::Vec<serde_json::Value>;
    // json!("abc");
    
    // let s = format!("name={}", "plucky");
    testargs(format_args!("name={}","plucky"))
}

fn testargs(args: std::fmt::Arguments){
    println!("args: {:?}",args);
}