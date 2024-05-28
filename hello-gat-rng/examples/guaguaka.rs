/*
 * @Author: plucky
 * @Date: 2023-05-20 10:34:51
 * @LastEditTime: 2023-09-19 22:23:44
 * @Description: 刮刮卡
 */

fn main() {
    let tickets = 100000.0;
    let award = [3000, 300, 30, 10, 3, 1]; // 奖项
    // [1, 30, 300, 3000, 3500, 35000]
    let winning: Vec<i32> = vec![1, 30, 300, 3000, 3500, 35500]; // 票数

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

    println!("剩余多少: {}", tickets - (total as f32));
    println!("期望利润: {}", tickets * 0.03);
    
   
}

    /*
        中奖数量: 42331
        中奖票数占比: 0.42330998
        没有中奖数量: 57669
        没有中奖占比: 0.57668996
        票数: [1, 30, 300, 3000, 3500, 35500]
        中奖概率: [0.000010, 0.000300, 0.003000, 0.030000, 0.035000, 0.355000]
        需要多少奖金: 97000
        剩余多少: 3000
        期望利润: 3000
     */

// 测试权重
#[test]
fn test_scratch_card(){
    use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};
    // 赔率
    let odds = vec![3000, 300, 30, 10, 3, 1];
    // 权重
    let weights = vec![1, 29, 600, 1670, 3500, 13000];
    // 中奖次数
    let mut 中奖次数 = vec![0; odds.len()];

    
    // 权重总和
    let sum = weights.iter().sum::<i32>();
    // 赔率*权重 总和
    let total = odds.iter()
        .zip(weights.iter())
        .map(|(x, y)| x * y)
        .sum::<i32>();

    // 倍率*权重 所有项总和/权重总和 = 平均赔率
    // 平均赔率
    let avg = total  as f64 / sum  as f64;
    let rtp = 0.98 / avg;
    // 真实击中率 = rtp/平均赔率
    println!("平均倍数: {}", avg);
    println!("真实击中率: {}", rtp);
    
    let chance:Vec<f64> = weights.iter()
        .map(|v| *v as f64 / sum  as f64)
        .collect();
    println!("权重总和: {}", sum);
    // println!("赔率{:?}", odds);
    println!("概率{:.6?}", chance);
    
    let mut free_card = 0;
    // 权重生成
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::thread_rng();

    let times = 1000;
    let mut n = times;
    // 购买100000次
    while n > 0 {
        n -= 1;
        let r = rng.gen::<f64>();
        if r < rtp {
            // 中奖了,看哪个奖项
            let idx = dist.sample(&mut rng);
           
            if idx == 5 {
                let r = rng.gen::<f64>();
                if r < 0.05{
                    n +=1;
                    free_card += 1;
                } else {
                    中奖次数[idx] += 1;
                }
            }else {
                中奖次数[idx] += 1;
            }
        }else {
            // 没中奖, 免费卡
            let r = rng.gen::<f64>();
            if r < 0.02{
                n +=1;
                free_card += 1;
            }
            
        }
    }

    let total = odds.iter().zip(中奖次数.iter()).map(|(x, y)| {
        x * y
    }).sum::<i32>();
    let hit_times = 中奖次数.iter().sum::<i32>();

    println!("中奖等级: {:?}", odds);
    println!("中奖次数: {:?}", 中奖次数);
    println!("中奖票数: {:?}", hit_times);
    println!("中奖概率: {:?}", hit_times as f64 / times as f64);
    println!("奖金总数: {}", total);
    println!("奖金占比: {}", total as f64 / times as f64);
    println!("免费卡数: {}", free_card);

    
}

/*
    均倍数: 4.12625
    真实击中率: 0.24235080278703425
    权重总和: 16000
    概率[0.000063, 0.003687, 0.031250, 0.062500, 0.183750, 0.718750]
    中奖等级: [3000, 300, 30, 10, 3, 1]
    中奖次数: [1, 81, 786, 1489, 4616, 17466]
    中奖票数: 24439
    中奖概率: 0.24439
    奖金总数: 97084
*/