/*
 * @Author: anger
 * @Date: 2022-06-24 09:50:22
 * @LastEditTime: 2022-06-24 18:11:21
 * @Description: 
 */

///关联类型的trait
pub trait Iterator{
    type Item;//Item 是一个占位类型, 在实现 trait的时候需要指定一个类型
    fn next(&mut self)->Option<Self::Item>;
}
struct Counter{}

// 关联类型只能实现一次
impl Iterator for Counter {
    type Item = u32;//指定Item是u32
    fn next(&mut self)->Option<Self::Item>{
        Some(1)
    }
}

///泛型trait,可以实现多个类型的trait
pub trait Iterator2<T>{
    fn next(&mut self)->Option<T>;
}
#[test]
fn test_item_type() {
    
}