// 测试火箭加速度模拟
#[test]
fn rocket_acceleration() {
    let v0 = 0.0f32; // 初始速度
    let y0 = 1.01f32; // 初始位置
    let a = 0.5f32; // 正常加速度
    let threshold = 3.0f32; // 阈值,超过这个值使用指数增长
    let exp_base = 1.9f32; // 指数基数

    // if t <= threshold
    //   y = y0 + v0 * t + a * t.powi(2)
    // else
    //   let y_threshold = y0 + v0 * threshold + a * threshold.powi(2)
    //   y = y_threshold * (exp_base.powf(t - threshold))

    println!("火箭加速度模拟:");
    println!("时间\t位置\t说明");

    for i in 0..40 {
        let t = i as f32 * 0.2; // 更小的时间间隔以观察变化
        let y = if t <= threshold {
            // 2.0之前使用正常加速度公式，系数为1.0
            // y = y0 + v0 * t + a * t^2
            y0 + v0 * t + a * t.powi(2)
        } else {
            // 2.0之后使用指数增长
            // y_threshold = y0 + v0 * threshold + a * threshold.powi(2)
            // y = y_threshold * (exp_base.powf(t - threshold))
            // 先计算到阈值时的位置
            let y_threshold = y0 + v0 * threshold + a * threshold.powi(2);
            // 然后从该位置开始指数增长
            y_threshold * (exp_base.powf(t - threshold))
        };

        let note = if t <= threshold {
            "正常加速"
        } else {
            "指数加速"
        };

        println!("{:.1}\t{:.4}\t{}", t, y, note);
    }
}

#[test]
fn test_position_to_time() {
    let v0 = 0.0f32; // 初始速度
    let y0 = 1.01f32; // 初始位置
    let a = 0.1f32; // 正常加速度
    let threshold = 3.0f32; // 阈值
    let exp_base = 1.9f32; // 指数基数

    println!("位置反推时间测试:");
    println!("原始位置\t计算时间\t验证位置\t误差");

    // 测试一系列位置
    let test_positions = [1.05, 1.1, 1.2, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0];

    for &test_y in &test_positions {
        // 反推时间
        let calculated_t = position_to_time(test_y, y0, v0, a, threshold, exp_base);

        // 用计算出的时间再算回位置进行验证
        let verified_y = if calculated_t <= threshold {
            y0 + v0 * calculated_t + a * calculated_t.powi(2)
        } else {
            let y_threshold = y0 + v0 * threshold + a * threshold.powi(2);
            y_threshold * (exp_base.powf(calculated_t - threshold))
        };

        // 计算误差
        let error = (test_y - verified_y).abs();

        println!("{:.4}\t{:.4}\t{:.4}\t{:.8}", test_y, calculated_t, verified_y, error);
    }
}

fn position_to_time(y: f32, y0: f32, v0: f32, a: f32, threshold: f32, exp_base: f32) -> f32 {
    // 计算阈值时刻的位置
    let y_threshold = y0 + v0 * threshold + a * threshold.powi(2);

    if y <= y_threshold {
        // 正常加速阶段
        // 解二次方程：a * t² + v0 * t + (y0 - y) = 0
        let discriminant = v0.powi(2) + 4.0 * a * (y - y0);
        if discriminant < 0.0 {
            panic!("无法到达该位置");
        }

        let t1 = (-v0 + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-v0 - discriminant.sqrt()) / (2.0 * a);

        // 返回有效的正时间值
        if t1 >= 0.0 && t1 <= threshold {
            t1
        } else if t2 >= 0.0 && t2 <= threshold {
            t2
        } else {
            panic!("计算出的时间超出正常加速阶段范围");
        }
    } else {
        // 指数加速阶段
        threshold + (y / y_threshold).ln() / exp_base.ln()
    }
}

#[test]
fn test1() {
    // 加速度公式：y = y0 + v0 * t + 0.5 * a * t^2
    // y代表物体在直线上的位置，y0代表物体的初始位置，v0代表物体的初始速度，a代表物体在直线上的加速度，t代表时间
    let v0 = 0.0;
    #[allow(unused)]
    let mut t = 0.0f32;
    let a = 0.05;
    let y0 = 1.01;

    for i in 1..30 {
        t = i as f32;
        println!("t={}, y={}", t, v0 * t + 0.5 * a * t.powi(2) + y0);
    }
    // 加速度公式,y是赔率,t是时间
    // y = 1.01 + 0.5 * 0.02 * t^2
    // 根据y,求t
    // t = ((y-1.01)/(0.5 * 0.02)).sqrt()
    let y = 1.01f32;
    let t = ((y - y0) / (0.5 * a)).sqrt();
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
