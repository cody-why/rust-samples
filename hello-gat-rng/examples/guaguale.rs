/*
 * @Author: plucky
 * @Date: 2023-05-20 10:34:51
 * @LastEditTime: 2024-04-12 16:05:01
 * @Description: 刮刮卡
 */



fn main() {
    let tickets = 100000.0;
    let price = 10.0;
    let award = [30000, 3000, 300, 100, 30, 10]; // 奖项
    // [1, 30, 300, 3000, 3500, 35000]
    let winning: Vec<i32> = vec![1, 30, 300, 3000, 3500, 35000]; // 票数

    let total = award.iter().zip(winning.iter()).map(|(a, b)| {
            a * b
        }).sum::<i32>();
    let winnings: i32 = winning.iter().sum();
    // 中奖概率
    let probability: Vec<f32> = winning.iter().map(|v| {
            (*v as f32) / tickets
        }).collect();

    println!("中奖数量: {}", winnings);
    println!("中奖票数占比: {:?}", probability.iter().sum::<f32>());
    println!("没有中奖数量: {}", tickets as i32 - winnings);
    println!("没有中奖占比: {}", 1.0-winnings as f32 / tickets);
    println!("票数: {:?}", winning);
    println!("中奖概率: {:.6?}", probability);
    
    println!("需要多少奖金: {}", total);
    let all = tickets * price;

    println!("剩余多少: {}", all - (total as f32));
    println!("期望利润: {}", all * 0.03);

    // 奖数量: 41831
    // 中奖票数占比: 0.41831
    // 没有中奖数量: 58169
    // 没有中奖占比: 0.58169
    // 票数: [1, 30, 300, 3000, 3500, 35000]
    // 中奖概率: [0.000010, 0.000300, 0.003000, 0.030000, 0.035000, 0.350000]
    // 需要多少奖金: 965000
    // 剩余多少: 35000
    // 期望利润: 30000
   
}


// 测试平均击中率
#[test]
fn test_avg() {
    // 赔率
    let odds = vec![3000, 300, 30, 10, 3, 1];
    // 权重
    let weights = vec![1, 59, 500, 1000, 2500, 7000];
    // 权重总和
    let sum = weights.iter().sum::<i32>() as f32;
    // 赔率*权重 总和
    let total = odds.iter()
        .zip(weights.iter())
        .map(|(x, y)| x * y)
        .sum::<i32>() as f32;

    // 倍率*权重 所有项总和/权重总和 = 平均赔率
    // 平均赔率
    let avg = total / sum;
    
    // 真实击中率 = rtp/平均赔率
    println!("平均倍数: {}", avg);
    println!("真实击中率: {}", 0.98 / avg);

    // 平均倍数: 22.960562
    // 真实击中率: 0.042681884

    let chance:Vec<f32> = weights.iter()
        .map(|v| *v as f32 / sum)
        .collect();
    println!("权重总和: {}", sum);
    // println!("赔率{:?}", odds);
    println!("概率{:.6?}", chance);
    
// 平均倍数: 13.332006
// 真实击中率: 0.07350732
// 权重总和: 10051
// 概率[0.000099, 0.004975, 0.248731, 0.348224, 0.099493, 0.298478]

}

// 测试权重
#[test]
fn weighted_index() {
    use rand::{distributions::WeightedIndex, prelude::Distribution};
    // 赔率
    let odds = vec![3000, 300, 30, 10, 3, 1];
    // 权重
    let weights = vec![1, 59, 500, 1500, 3000, 6000];
    let mut total = vec![0; odds.len()];

    // 权重生成
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::thread_rng();
    
    for _i in 0..1000 {
        let idx = dist.sample(&mut rng);
        // println!("Selected item: {}",  pl[idx]);
        total[idx] += 1;
    }

    odds.iter().zip(total.iter()).for_each(|(x, y)| {
        println!("{}: {}", x, y);
    });
    
}


#[test]
fn test_scratch_card(){
    use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};
    // 赔率
    let odds = vec![3000, 300, 30, 10, 3, 1];
    // 权重
    let weights = vec![1, 50, 600, 500, 400, 400];
    // 中奖次数
    let mut total = vec![0; odds.len()];

    // 权重生成
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::thread_rng();

    // 购买10000次
    for _i in 0..10000 {
        let rtp = 0.044681884;
        let r = rng.gen::<f64>();
        if r < rtp {
            // 中奖了,看哪个奖项
            let idx = dist.sample(&mut rng);
            total[idx] += 1;
        }
    }

    let total = odds.iter().zip(total.iter()).map(|(x, y)| {
        println!("{}: {}", x, y);
        x * y
    }).sum::<i32>();

    println!("奖金总数: {}", total);
    println!("奖金占比: {}", total as f32 / 10000.0);

    
}