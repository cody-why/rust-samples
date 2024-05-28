/*
 * @Author: plucky
 * @Date: 2023-05-24 22:43:25
 * @LastEditTime: 2023-05-24 23:14:06
 * @Description: 
 */

// 状态模式

use std::{fmt::{ Formatter,Error, Debug}};

trait TrafficSignalState {
    fn duration(&self) -> u8;
    fn next(&self) -> Box<dyn TrafficSignalState>;
}


 enum TrafficSignal {
    Red(Box<dyn TrafficSignalState>),
    Yellow(Box<dyn TrafficSignalState>),
    Green(Box<dyn TrafficSignalState>)
}
impl Debug for TrafficSignal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            TrafficSignal::Red(_) => write!(f, "Red"),
            TrafficSignal::Yellow(_) => write!(f, "Yellow"),
            TrafficSignal::Green(_) => write!(f, "Green")
        }
    }
}

 struct RedSignalState;
 impl TrafficSignalState for RedSignalState {
    fn duration(&self) -> u8 {
        3
    }
     fn next(&self) -> Box<dyn TrafficSignalState> {
        Box::new(YellowSignalState)
    }
}
 struct YellowSignalState;
 impl TrafficSignalState for YellowSignalState {
    fn duration(&self) -> u8 {
        2
    }
     fn next(&self) -> Box<dyn TrafficSignalState> {
        Box::new(GreenSignalState)
    }
}
 struct GreenSignalState;
 impl TrafficSignalState for GreenSignalState {
    fn duration(&self) -> u8 {
        4
    }
     fn next(&self) -> Box<dyn TrafficSignalState> {
        Box::new(RedSignalState)
    }
}
 impl TrafficSignal {
    fn new() -> TrafficSignal {
        TrafficSignal::Red(Box::new(RedSignalState))
    }
     fn duration(&self) -> u8 {
        match self {
            TrafficSignal::Red(state) => state.duration(),
            TrafficSignal::Yellow(state) => state.duration(),
            TrafficSignal::Green(state) => state.duration()
        }
    }
     fn next(&mut self) {
        match self {
            TrafficSignal::Red(state) => {
                *self = TrafficSignal::Yellow(state.next());
            },
            TrafficSignal::Yellow(state) => {
                *self = TrafficSignal::Green(state.next());
            },
            TrafficSignal::Green(state) => {
                *self = TrafficSignal::Red(state.next());
            }
        }
    }
}

 fn main() {
    let mut signal = TrafficSignal::new();
    
    let mut i = 0;
    let mut n = 0;
    while n < 3 {
        println!("Signal State: {:?}, Time left: {}", signal, signal.duration() - i);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if i >= signal.duration()-1 {
            signal.next();
            i = 0;
            n += 1;
        } else {
            i += 1;
        }
        
    }
}

// 在上面的代码中，我们首先定义了一个  TrafficSignalState  trait，它包含了两个方法： duration  和  next 。然后我们定义了三种交通信号灯状态的结构体： RedSignalState 、 YellowSignalState  和  GreenSignalState ，它们都实现了  TrafficSignalState  trait，分别对应着红灯、黄灯和绿灯状态。 
 
// 我们还定义了一个  TrafficSignal  枚举，表示交通信号灯。它有三个变量： Red 、 Yellow  和  Green ，每个变量都是一个包含状态对象的  Box<dyn TrafficSignalState>  类型。 
 
// 在  TrafficSignal  中，我们通过实现  duration  和  next  方法来委托给当前状态来处理。在  next  方法中，我们获取当前状态对象并调用其  next  方法来获取下一个状态，并将其封装到一个新的  TrafficSignal  变量中。 
 
// 最后，在我们的示例中，我们创建了一个  TrafficSignal  对象并循环输出其状态和时间。当时间等于持续时间时，我们调用  next  方法来切换状态。