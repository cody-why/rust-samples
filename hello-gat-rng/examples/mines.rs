/*
 * @Date: 2023-12-26 19:03:58
 * @LastEditTime: 2023-12-27 15:47:59
 */


fn main() {
    let zhadan = 5; // 炸弹数量(1-20)
    let amount = 100.0; // 金额
    let rtp = 0.97;
    let total = 25; // 总格子数
    let xingxing = total - zhadan; // 星星数量
    let mut gailv = 1.0; // 概率

    // 只能玩次数:格子数-炸弹数
    for i in 0..total - zhadan {
        // 上次的概率*剩余星星数/剩余格子数
        gailv = gailv *  (xingxing - i) as f64  / (total - i) as f64;
        println!("概率: {}",  gailv);
        // 计算派奖金额
        let reward = (rtp / gailv) * amount;
        // 输出2位小数
        println!("第{}次=: {:.2}", i+1, reward);
    }

    // let probability = rtp * 0.96;
    // // 随机100次, 出现概率的次数
    // let mut count = 0;
    // for _ in 0..100 {
    //     let mut rng = rand::thread_rng();
    //     let random = rng.gen_range(0.0..1.0);
    //     if random < probability {
    //         count += 1;
    //     }
    // }
    // println!("出现概率的次数: {}", count);

}