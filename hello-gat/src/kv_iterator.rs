/*
 * @Author: plucky
 * @Date: 2022-11-24 21:56:49
 * @LastEditTime: 2022-11-25 12:06:11
 * @Description: 1.65 nightly
 */
 
// #![feature(generic_associated_types)]
// #![feature(type_alias_impl_trait)]

use std::future::Future;

pub trait KvIterator {
    type NextFuture<'a>: Future<Output = Option<(&'a [u8], &'a [u8])>>
    where
        Self: 'a;

    /// Get the next item from the iterator.
    fn next(&mut self) -> Self::NextFuture<'_>;
}


pub struct TestIterator {
    idx: usize,
    to_idx: usize,
    key: Vec<u8>,
    value: Vec<u8>,
}

impl TestIterator {
    pub fn new(from_idx: usize, to_idx: usize) -> Self {
        Self {
            idx: from_idx,
            to_idx,
            key: Vec::new(),
            value: Vec::new(),
        }
    }
}

impl KvIterator for TestIterator {
    // GAT
    type NextFuture<'a> = impl Future<Output = Option<(&'a [u8], &'a [u8])>>
    where
        Self: 'a;


    fn next(&mut self) -> Self::NextFuture<'_> {
        use std::io::Write as _;
        async move {
            if self.idx >= self.to_idx {
                return None;
            }

            // Zero-allocation key value manipulation

            self.key.clear();
            write!(&mut self.key, "key_{:05}", self.idx).unwrap();

            self.value.clear();
            write!(&mut self.value, "value_{:05}", self.idx).unwrap();

            self.idx += 1;
            Some((&self.key[..], &self.value[..]))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_iterator() {
        use bytes::Bytes;
        let mut iter = TestIterator::new(0, 10);
        while let Some((key, value)) = iter.next().await {
            println!(
                "{:?} {:?}",
                Bytes::copy_from_slice(key),
                Bytes::copy_from_slice(value)
            );
        }
    }
}