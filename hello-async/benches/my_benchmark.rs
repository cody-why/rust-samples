/*
 * @Author: plucky
 * @Date: 2022-06-25 22:05:19
 * @LastEditTime: 2023-07-07 09:28:08
 * @Description: 
 */

//! 用criterion进行benchmark测试的例子
//! cargo bench --bench my_benchmark

use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// 斐波纳契函数
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}
// 一个测试组
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);