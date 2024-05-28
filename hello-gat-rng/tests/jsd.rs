/*
 * @Date: 2024-04-12 15:52:58
 * @LastEditTime: 2024-04-23 16:18:21
 */

#[test]
fn test1() {
    // 加速度公式：y = y0 + v0 * t + 0.5 * a * t^2
    // y代表物体在直线上的位置，y0代表物体的初始位置，v0代表物体的初始速度，a代表物体在直线上的加速度，t代表时间
    let v0=0.0;
    #[allow(unused)]
    let mut t=0.0f32;
    let a=0.05;
    let y0=1.01;

    for i in 1..30 {
        t = i as f32;
        println!("t={}, y={}",t, v0 * t + 0.5 * a * t.powi(2) + y0);
    }
    // 加速度公式,y是赔率,t是时间
    // y = 1.01 + 0.5 * 0.02 * t^2
    // 根据y,求t
    // t = ((y-1.01)/(0.5 * 0.02)).sqrt()
    let y = 1.01f32;
    let t = ((y - y0)/(0.5 * a)).sqrt();
    println!("y={}, t={}", y, t);
}

// 测试线性概率
#[test]
fn feature2() {
    // y = 0.1 + 0.5 X 0.1 X t^2;
    let v0 = 0.03f32; // 初速度
    let y0 = 0.00f32; // 初始位置
    let a = 0.02; // 加速度
    for i in 1..11 {
        let t = i as f32;
        let y = v0 * t + 0.5 * a * t.powi(2) + y0;
        println!("t={}, y={}", t, y);
    }
}