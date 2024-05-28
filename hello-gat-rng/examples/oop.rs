/*
 * @Author: plucky
 * @Date: 2022-12-19 22:14:41
 * @LastEditTime: 2022-12-19 22:19:10
 * @Description: 
 */

use std::ops::Deref;

fn main(){
    
}

#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

// 通过实现Deref trait，继承了Point
struct MyPoint(Point);

impl Deref for MyPoint {
    type Target = Point;

    fn deref(&self) -> &Point {
        &self.0
    }
}