/*
 * @Author: plucky
 * @Date: 2022-11-28 18:07:47
 * @LastEditTime: 2022-12-01 21:01:36
 * @Description:
 */

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_name() {
        // 倍率*各自权重 再总和/所有权重总和 = 平均赔率
        // 貂蝉平均赔率=92.2 真实概率=rtp/平均赔率 =0.010624746
        let pl = vec![60, 64, 72, 75, 80, 90, 96, 100, 108, 120, 125, 144, 150, 180, 216];
        let qz = vec![80, 80, 80, 80, 80, 80, 80, 50, 50, 30, 30, 30, 20, 20, 10];
        // 88.4
        // let pl = vec![60, 70, 80, 85, 90, 95, 100, 110, 120, 130, 140, 150];
        // let qz = vec![120, 120, 120, 120, 100, 100, 100, 50, 50, 20, 20, 10];

        // 权重总和
        let sum = qz.iter().sum::<i32>() as f32;
        // 赔率*权重 总和
        let total = pl
            .iter()
            .zip(qz.iter())
            .map(|(x, y)| x * y)
            .sum::<i32>() as f32;
        // 平均击中率
        let avg = total / sum;
        // 真实击中率 = rtp/平均击中率
        println!("avg: {}", avg);
        println!("rtp: {}", 0.98 / avg);
        
    }

    #[test]
    fn test_rang() {
        // 假设击杀率是0.01
        let sjl:f32 = 0.01;
        // 4倍击杀率
        let m = 4;
        // 真实击杀率
        let real = 1.0-(1.0-sjl).powi(m);
        println!("real: {}", real*1.0);

        let mut rng = rand::thread_rng();
        let mut count = 0;
        let mut first = 0;
        for i in 0..10000 {
            let r = rand::Rng::gen_range(&mut rng, 0.0..1.0);
            if r < real {
                count += 1;
                if first == 0 {
                    first = i;
                }
            }
        }
        println!("count: {}", count);
        println!("first: {}", first);


    }
}