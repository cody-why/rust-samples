///单元测试
#[cfg(test)]
mod tests {
    //use super::*;
    use crate::display::*;

    #[test]
    fn print_list() {
        assert_eq!(4, 2 * 2);
        let v = dis::List(vec![1, 2, 3]);
        //实现了display的输出
        println!("{}", v);
        //宏debug的输出
        println!("{:?}", v);
    }
}
