/*
 * @Author: plucky
 * @Date: 2022-11-16 20:12:52
 * @LastEditTime: 2022-11-16 21:09:09
 * @Description: 
 */

// 为Iterator实现自定义方法
// 首先定义一个struct保存上一个值,函数体,以及Iterator
struct AdjacentDifference<I, F, R>
where
    I: Iterator,
    F: Fn(I::Item, I::Item) -> R,
    R: From<I::Item>
{
    prev: Option<I::Item>,
    it: I,
    f: F,
}

// 实现Iterator for 自定义结构体
impl<I, F, R> Iterator for AdjacentDifference<I, F, R>
where
    I: Iterator,
    I::Item: Copy,
    F: Fn(I::Item, I::Item) -> R,
    R: From<I::Item>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|i| {
            // 如果没有值第一个默认函数，或 如果有值计算第二个函数。
            let r = self.prev.map_or_else(|| i.into(), |p| (self.f)(i, p));
            self.prev = Some(i);
            r
        })
    }
}

// 为自定义结构体增加 Iterator 方法
trait AdjacentDifferenceExt: Iterator
where
    Self: Sized,
{
    fn adjacent_difference<F, R>(self, f: F) -> AdjacentDifference<Self, F, R>
    where
        <Self as Iterator>::Item: Copy,
        F: Fn(<Self as Iterator>::Item, <Self as Iterator>::Item) -> R,
        R: From<<Self as Iterator>::Item>
    {
        AdjacentDifference {
            prev: None,
            it: self,
            f
        }
    }
}

// 为所有实现了Iterator的类型增加adjacent_difference方法
impl<I> AdjacentDifferenceExt for I where I: Iterator {}

fn main() {
    let v = [1, 7, 3, 5, 7, 2, 1];
    for i in v.iter().copied().adjacent_difference(|a, b| a - b) {
        println!("{}", i);
    }
}