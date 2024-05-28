/*
 * @Author: plucky
 * @Date: 2023-05-26 01:26:46
 * @LastEditTime: 2023-05-26 01:27:59
 * @Description: 
 */

// 迭代器模式

struct Range {
    start: i32,
    end: i32,
}
 impl Iterator for Range {
    type Item = i32;
     fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let result = Some(self.start);
            self.start += 1;
            result
        } else {
            None
        }
    }
}
 fn main() {
    let range = Range { start: 0, end: 5 };
     for num in range {
        println!("{}", num);
    }
}

// 迭代器模式是一种设计模式，其中一个对象聚合了一系列元素，并一次提供访问一个元素的访问方式，而不暴露其底层表示。这种模式用于允许客户端代码遍历对象集合而不必知道底层数据结构的细节。 
// 在Rust中，标准库通过 Iterator  trait提供了对迭代器模式的内置支持。该trait提供了几种方法，可用于构建、转换和消耗一系列元素。 
// 要实现 Iterator  trait，需要定义一个关联类型 Item ，表示迭代器返回的元素类型，并实现 next 方法，该方法返回一个 Option<Item> ，表示序列中的下一个元素或 None 如果没有更多元素。