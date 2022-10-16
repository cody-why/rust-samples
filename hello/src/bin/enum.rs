#[derive(Debug)]
enum TrafficLightColor {
    #[allow(unused)]
    Red,
    Yellow,
    #[allow(unused)]
    Green,
}

// 为 TrafficLightColor 实现所需的方法
impl TrafficLightColor {
    fn color(&self) -> &str {
        //let 匹配
        if let TrafficLightColor::Red = self {
            return "red";
        }
        //match 匹配
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Green => "green",
        }
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}
