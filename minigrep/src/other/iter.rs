/// 一个累加数
struct Counter {
    count: u32,
}

#[allow(dead_code)]
impl Counter {
    /// Creates a new [`Counter`].
    fn new() -> Self {
        Self { count: 0 }
    }
}

/// 为他实现迭代器
impl Iterator for Counter {
    //别名
    type Item = u32;
    /// 所有迭代都是通过next实现
    ///
    fn next(&mut self) -> Option<Self::Item> {
        //限定累加到5吧
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn testiter() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.skip(4).next(), None);
}

#[test]
fn test_iter_trait() {
    //zip拉链把a和b组成元组[(a1,b1),(a2,b2)...]
    //map将一个迭代器转换为另一个迭代器,通过表达式对参数做一下操作
    //filter新的迭代器,闭包返回“true”保留或“false”去掉
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
