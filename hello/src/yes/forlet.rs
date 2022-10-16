/*
 * @Author: anger
 * @Date: 2022-06-23 21:06:47
 * @LastEditTime: 2022-06-23 22:24:24
 * @Description:
 */
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_for() {
        let mut v = vec![1, 2, 3];
        while let Some(i) = v.pop() {
            println!("{}", i);
        }
        let v = vec![5, 2, 3];
        if let Some(i) = v.get(0) {
            println!("{}", i);
        }
        let v = vec![7, 8, 9];
        for i in &v {
            println!("{}", i);
        }
        let v = vec![1, 2, 3];
        for (i, v) in v.iter().enumerate() {
            println!("{}={}", i, v);
        }
    }

    #[test]
    fn test_math() {
        struct Point {
            x: u32,
            y: u32,
        }

        let p = Point { x: 3, y: 0 };
        match p {
            Point { x, y: 0 } => println!("match y is zero,x={}", x),
            Point { x: 0, y } => println!("match x is zero,y={}", y),
            Point { x, y } => println!("match x={},y={}", x, y),
        }
        let x = 5;
        match x {
            1 => println!("x is 1"),
            0..=5 => println!("match 0-5"),
            _ => println!("other something"),
            
        }

        enum OneOrTwo {
            One(i32),
            Two,
        }
        let _t = OneOrTwo::Two;
        let t = OneOrTwo::One(100);
        match t {
            OneOrTwo::One(x) if x<100 => println!("match one and x<100, x={}", x),
            OneOrTwo::One(x) => println!("match one x={}", x),
            OneOrTwo::Two => println!("match two"),
        }
    }

    #[test]
    fn test_let() {
        let s = Some(String:: from( "Hello!"));
        // Some(a)有变量a会发生所有权转移
        if let Some(_) = s{
            println! ( "found a string");

        }
        println!("{:?}",s);
        
    }
}
