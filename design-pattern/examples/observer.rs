/*
 * @Author: plucky
 * @Date: 2023-05-24 21:28:17
 * @LastEditTime: 2023-05-24 22:12:12
 * @Description: 
 */
// 观察者模式
use std::rc::Rc;
 trait Observer {
    fn update(&self, temperature: f32);
}
 struct Display1;
 impl Observer for Display1 {
    fn update(&self, temperature: f32) {
        println!("Display1: {}", temperature);
    }
}
 struct Display2;
 impl Observer for Display2 {
    fn update(&self, temperature: f32) {
        println!("Display2: {}", temperature);
    }
}
 trait Subject {
    fn add_observer(&mut self, observer: Rc<dyn Observer>);
    fn remove_observer(&mut self, observer: Rc<dyn Observer>);
    fn notify_observers(&self);
}
 struct WeatherData {
    temperature: f32,
    observers: Vec<Rc<dyn Observer>>,
}
 impl WeatherData {
    fn new() -> Self {
        WeatherData {
            temperature: 0.0,
            observers: Vec::new(),
        }
    }
     fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
        self.notify_observers();
    }
}
 impl Subject for WeatherData {
    fn add_observer(&mut self, observer: Rc<dyn Observer>) {
        self.observers.push(observer);
    }
     fn remove_observer(&mut self, observer: Rc<dyn Observer>) {
        self.observers.retain(|obs| Rc::ptr_eq(obs, &observer));
    }
     fn notify_observers(&self) {
        for observer in &self.observers {
            observer.update(self.temperature);
        }
    }
}
 fn main() {
    let mut weather_data = WeatherData::new();
    let display1 = Rc::new(Display1 {});
    let display2 = Rc::new(Display2 {});
    weather_data.add_observer(display1.clone());
    weather_data.add_observer(display2.clone());
    weather_data.set_temperature(25.5);
    weather_data.remove_observer(display1.clone());
    weather_data.set_temperature(30.0);
}
// 观察者模式定义了一种一对多的依赖关系，让多个观察者对象同时监听某一个主题对象。当主题对象状态发生变化时，它会通知所有观察者对象，使它们能够自动更新自己。 
// 观察者模式可以帮助我们实现松耦合的设计，因为主题和观察者彼此之间不直接相互依赖，它们只是通过抽象的接口进行通信。这使得我们可以很容易地添加或移除观察者对象，而不会对主题对象或其他观察者对象造成任何影响。