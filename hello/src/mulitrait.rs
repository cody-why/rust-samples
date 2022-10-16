// 多trait歧义的演示

trait A {
    fn test(&self, i: i32) {
        println!("from trait A: {:?}", i);
    }
}
trait B {
    fn test(&self, i: i32) {
        println!("from trait B: {:?}", i);
    }
}

struct S(i32);

//S 实现了2个trait,就有2个test()
impl A for S {}
impl B for S {}

#[cfg(test)]
mod tests {
    use super::*;
    //无歧义完全限定语法
    #[test]
    fn test() {
        let s = S(1);
        //s.test();//歧义
        //结合as关键字可以避免歧义
        <S as A>::test(&s, 10);
        <S as B>::test(&s, 11);
    }
}
