#[cfg(test)]
mod test_random {
    use rand::{distributions::Uniform, prelude::Distribution};
    /// 测试随机值分布,从6个数里有3种颜色,a,b,c颜色,抽到某个颜色,2个颜色,3个颜色的统计
    //a=9988, b=9918, c=10094, a2=2617, b2=2534, c2=2630, a3=379, b3=359, c3=390
    #[test]
    fn test_random_colour() {
        let max = 10000;
        let mut rng = rand::thread_rng();
        //创建一个均匀分布的值,再随机抽样
        let between = Uniform::from(0..6);
        //6个值有3种颜色
        let sz = [1, 2, 3, 3, 2, 1];

        let mut all: Vec<[i32; 3]> = Vec::with_capacity(max);

        //10000次抽样,每次抽3个数
        for _i in 0..max {
            let mut v = [0, 0, 0];
            //一次抽3个数
            for j in 0..3 {
                let r = between.sample(&mut rng);
                //let r = rng.gen_range(0..6);
                //print!("{} ",r);
                v[j] = sz[r];
            }

            all.push(v);
            //println!("{:?}",v);
        }

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut a2 = 0;
        let mut b2 = 0;
        let mut c2 = 0;
        let mut a3 = 0;
        let mut b3 = 0;
        let mut c3 = 0;

        //统计
        for i in 0..max {
            let mut mn = all[i];
            mn.sort();

            //println!("{:?}",mn);

            for j in 0..3 {
                match mn[j] {
                    1 => a += 1,
                    2 => b += 1,
                    3 => c += 1,
                    _ => (),
                }
            }

            //2个相同
            if mn[0] == mn[1] {
                match mn[0] {
                    1 => a2 += 1,
                    2 => b2 += 1,
                    3 => c2 += 1,

                    _ => (),
                }
                //3个相同
                if mn[1] == mn[2] {
                    match mn[0] {
                        1 => a3 += 1,
                        2 => b3 += 1,
                        3 => c3 += 1,

                        _ => (),
                    }
                }
                //或者另外2个相同
            } else if mn[1] == mn[2] {
                match mn[1] {
                    1 => a2 += 1,
                    2 => b2 += 1,
                    3 => c2 += 1,

                    _ => (),
                }
            }
        }

        println!(
            "a={}, b={}, c={}, a2={}, b2={}, c2={}, a3={}, b3={}, c3={}",
            a, b, c, a2, b2, c2, a3, b3, c3
        );
    }

    //100次*每次要200=20000,中60次*100*2.85=17100
    //tx1=3296, tx2=3337, tx3=3367, ys1=2445, ys2=2539, ys3=2518, ys4=2498
    #[test]
    fn test_random_tuxing() {
        let max = 10000;
        let mut rng = rand::thread_rng();
        //3种图像
        let between = Uniform::from(0..3);
        //4种颜色
        let between2 = Uniform::from(0..4);

        let mut all: Vec<[i32; 2]> = Vec::with_capacity(max);

        //10000次抽样,每次抽1个图像,1个颜色
        for _i in 0..max {
            let t = between.sample(&mut rng);
            let r = between2.sample(&mut rng);
            let v = [t, r];
            all.push(v);
        }
        //图像
        let mut tx = [0, 0, 0];
        //颜色
        let mut ys = [0, 0, 0, 0];

        for i in 0..max {
            let v = all[i];
            match v[0] {
                0 => tx[0] += 1,
                1 => tx[1] += 1,
                2 => tx[2] += 1,
                _ => (),
            }
            match v[1] {
                0 => ys[0] += 1,
                1 => ys[1] += 1,
                2 => ys[2] += 1,
                3 => ys[3] += 1,
                _ => (),
            }
        }

        println!(
            "tx1={}, tx2={}, tx3={}, ys1={}, ys2={}, ys3={}, ys4={} ",
            tx[0], tx[1], tx[2], ys[0], ys[1], ys[2], ys[3]
        );
    }


    #[test]
    fn test_random_sedie() {
        let max = 10000;
        let mut rng = rand::thread_rng();

        //4个数字:1到4,抽2个颜色:0,1
        let between = Uniform::from(0..2);
       

        let mut all: Vec<[i32; 4]> = Vec::with_capacity(max);

        //10000次抽样,每次4个元素随机0或1
        for _i in 0..max {
            let mut v = [0,0,0,0];
            for i in 0..4 {
                let t = between.sample(&mut rng);
                v[i]=t;
            }
            
            all.push(v);
        }

        let mut red3 = 0;//3个红
        let mut red4=0;//4个红
        let mut white3=0;//3个白
        let mut white4=0;//4个白

        for i in 0..max {
            let v = all[i];
            let mut r =0;
            let mut w = 0;

            for j in 0..4 {
                match v[j] {
                    0 => r += 1,
                    1 => w += 1,
                    _ =>(),          
                }
            }
            
           match r {
               3 => red3+=1,
               4 => red4+=1,
               _ => (),
           }
           match w {
            3 => white3+=1,
            4 => white4+=1,
            _ => ()
        }
        }

        println!(
            "red3={}, red4={}, white3={}, white4={} ",
            red3, red4, white3, white4
        );
        let m = max as f32;

        println!(
            "red3={}, red4={}, white3={}, white4={} ",
            red3 as f32 /m, red4 as f32/m, white3 as f32/m, white4 as f32 /m
        );

    }

}
