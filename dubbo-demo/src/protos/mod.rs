/*
 * @Author: plucky
 * @Date: 2022-10-28 12:13:20
 * @LastEditTime: 2022-10-28 15:59:36
 * @Description: 
 */


pub mod greeter {
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/grpc.examples.greeter.rs"));
}

pub mod echo {
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/grpc.examples.echo.rs"));
}
