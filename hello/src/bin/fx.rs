//泛型

#[derive(Debug, Clone, Copy)]
struct Point<T, U> {
    x: T,
    y: U,
}

//这里是Point的泛型
impl<T, U> Point<T, U> {
    //这里是方法的泛型,传入参数的泛型
    fn mixup<A, B>(self, other: Point<A, B>) -> Point<T, B> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point {
        x: "Hello",
        y: '中',
    };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    //实现了Clone,p1才没有被转移
    println!("{:?}", p1);
}

//特征,复杂约束
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
//可以用where
#[allow(unused)]
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    0
}
