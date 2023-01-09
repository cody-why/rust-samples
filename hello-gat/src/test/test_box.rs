/*
 * @Author: plucky
 * @Date: 2022-12-04 12:03:15
 * @LastEditTime: 2022-12-04 21:22:46
 * @Description: https://course.rs/advance/smart-pointer/deref.html
 */

struct MyBox<T> {
    v: T,
}

#[allow(dead_code)]
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { v: x }
    }
}

use std::ops::Deref;
// Deref 特征的方法解引用为 &T
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

use std::ops::DerefMut;

// DerefMut 特征的方法解引用为 &mut U 类型
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        
        // 在栈上创建一个长度为1000的数组
        let arr = [0;1000];
        // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
        let arr1 = arr;
    
        // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
        println!("{:?}", arr.len());
        println!("{:?}", arr1.len());
    
        // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
        let arr = Box::new([0;1000]);
        // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
        // 所有权顺利转移给 arr1，arr 不再拥有所有权
        let arr1 = arr;
        println!("{:?}", arr1.len());
        // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
        // println!("{:?}", arr.len());

        let s = gen_static_str();
        println!("{}", s);
    }

    // 可以把一个 String 类型，变成一个 'static 生命周期的 &str 类型
    fn gen_static_str() -> &'static str{
        let mut s = String::new();
        s.push_str("hello, world");
        // Box::leak，它可以消费掉 Box 并且强制目标值从内存中泄漏,变成全局有效的
        //虽然 Rc/Arc 也可以实现此功能，但是 Box::leak 是性能最高的
        Box::leak(s.into_boxed_str())
    }

    #[test]
    fn test_box() {
        let mut b = MyBox::new(5);
        // *b 解引用为 &mut i32 类型
        assert_eq!(5, *b);
        // b.deref_mut() 解引用为 &mut i32 类型
        *b = 6;
        assert_eq!(6, *b);

        let mut s = MyBox::new(String::from("hello, "));
        // s.deref_mut() 解引用为 &mut String 类型
        display(&mut s)

    }

    fn display(s: &mut String) {
        s.push_str("world");
        println!("{}", s);
    }
    
}