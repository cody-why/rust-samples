/*
 * @Author: anger
 * @Date: 2022-06-05 22:18:38
 * @LastEditTime: 2022-06-23 23:22:16
 * @Description:
 */

mod memory;
mod mulitrait;
mod test_random;
mod thread;
mod high_level;
// lib.rs的函数,在外部调用crate::sort_bubble

/// 这是一个无用的例子,把引用包在mod里面,引用其他函数就不会引入use
mod sample {
    use rand::{distributions::Uniform, prelude::Distribution, Rng};

    /// 随机数的2种方法
    #[allow(dead_code)]
    pub fn sample_random() {
        let mut rng = rand::thread_rng();
        //创建一个均匀分布的值,再随机抽样
        let between = Uniform::from(1..3);
        let r = between.sample(&mut rng);
        println!("{}", &r);
        //范围随机
        let r = rng.gen_range(1..3);
        print!("{}", r);
    }
}

/// 冒泡排序
pub fn sort_bubble(vec: &mut Vec<i32>) {
    for i in 0..vec.len() {
        for j in 0..vec.len() - 1 - i {
            // 交换
            if vec[j] > vec[j + 1] {
                vec[j] = vec[j] ^ vec[j + 1];
                vec[j + 1] = vec[j] ^ vec[j + 1];
                vec[j] = vec[j] ^ vec[j + 1];
            }
        }
    }
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
