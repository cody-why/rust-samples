///泛型
///

/// 找出vec中最大数
#[allow(dead_code)]
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    *largest
}

#[allow(dead_code)]
#[derive(Debug)]
struct Point<T: Copy> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T: Copy> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }
}
#[test]
fn test1() {
    let list = vec![33, 28, 91, 59, 65];
    let l = largest(&list);
    println!("{}", l);

    let list = vec!['d', 'f', 'a'];
    let l = largest(&list);
    println!("{}", l);

    let p = Point::new(1, 3);

    println!("{:?}", p);
}
