use std::fmt;

/// 演示实现特性的方法,定义一个包含单个 `Vec` 的结构体 `List`。
#[derive(Debug)]
pub struct List(pub Vec<i32>);

//实现Display进行打印
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

pub fn display() {
    println!("from dis.rs!")
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_list() {
        let v = List(vec![1, 2, 3]);
        //实现了display的输出
        println!("{}", v);
        //宏debug的输出
        println!("{:?}", v);
    }
}
 */

//单元测试
// #[test]
// fn print_list11() {
//     assert_eq!(4, 2 * 2);
// }
