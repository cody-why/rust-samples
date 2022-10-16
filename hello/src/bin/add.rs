/***
 * @Author: plucky
 * @Date: 2022-06-16 00:38:25
 * @LastEditTime: 2022-09-09 21:40:50
 * @Description: 
 */

//泛型的理解,泛型进行运算需要特性

//Eq比较,Order排列特性
#[derive(PartialEq, PartialOrd)]
pub struct Bbc(i32);

//写法1,约束T实现了add
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
//写法2,用where约束
fn sum<T>(x: T, y: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    x + y
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
#[allow(dead_code)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("sum f64: {}", sum(1.23, 1.23));

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    println!("{:?}", number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("The points is {:?} / {:?}", integer, float);
}
