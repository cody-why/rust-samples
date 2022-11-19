#![allow(dead_code, unused)]
fn main(){

}

// 为自定义 struct 实现 Iterator, next()->first, second
#[derive(Debug, Default, Clone, std::hash::Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    first: i32,
    second: i32,
}

impl Pair {
    fn new(first: i32, second: i32) -> Self {
        Self { first, second}
    }
    // 实现iter
    fn iter(&self) -> Iter {
        Iter {pair: self, idx: 0}
    }
    // 实现iter_mut
    fn iter_mut(&mut self) -> std::iter::Chain<std::iter::Once<&mut i32>, std::iter::Once<&mut i32>> {
        use std::iter::once;
        once(&mut self.first).chain(once(&mut self.second))
    }
}

// 为Pair 实现 IntoIterator
impl IntoIterator for Pair {
    type Item = i32;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { pair: self, idx : 0}
    }
}

// IntoIter Pair的中间类型
struct IntoIter {
    pair: Pair, // 包装Pair
    idx: usize, // 记录迭代次数
}

// 实现 中间类型的 Iterator
impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let r = match self.idx {
            0 => self.pair.first,
            1 => self.pair.second,
            _ => return None
        };
        self.idx += 1;
        Some(r)
    }
}

// 实现 pair.into_iter
impl<'a> IntoIterator for &'a Pair {
    type Item = &'a i32;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter{ pair: self, idx : 0} }
}

// Pair的引用中间类型Iter
struct Iter<'a> {
    pair: &'a Pair,
    idx: usize,
}

// 实现 pair.iter
impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        let r = match self.idx {
            0 => &self.pair.first,
            1 => &self.pair.second,
            _ => return None
        };
        self.idx += 1;
        Some(r)
    }
}

// 实现 for i in &mut pair
impl<'a> IntoIterator for &'a mut Pair {
    type Item = &'a mut i32;

    type IntoIter = std::iter::Chain<std::iter::Once<&'a mut i32>, std::iter::Once<&'a mut i32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    // 测试自定义约定
    fn good_citizen<T>(_: T) where T: std::fmt::Debug + Default + Clone + std::hash::Hash + PartialEq + PartialOrd + Eq + Ord + Send + Sync {}
    #[test]
    fn being_good_citizen() { good_citizen(Pair::new(0, 0)) }

    #[test]
    fn user_defined() {
        let pair = Pair::new(1, 2);

        // for i int pair 等价 for i in pair.into_iter()
        for i in pair { let _ = i; }
        let pair = Pair::new(1, 2);
        for i in pair.into_iter() { let _ = i; }

        // for i in &pair 等价 for i in pair.iter()
        let pair = Pair::new(1, 2);
        for i in &pair { let _ = i; }
        let pair = Pair::new(1, 2);
        for i in pair.iter() { let _ = i; }

        // for i in &mut pair 等价 for i in pair.iter_mut()
        let mut pair = Pair::new(1, 2);
        for i in &mut pair { let _ = i; }
        let mut pair = Pair::new(1, 2);
        for i in pair.iter_mut() { let _ = i; }
    }
}