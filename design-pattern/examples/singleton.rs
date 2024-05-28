// 单例模式
struct Singleton {
    name: String,
    age: u8,
}
// 实现一个私有构造函数和一个公共访问单例实例的函数
impl Singleton {
    fn new(name: &str, age: u8) -> Singleton {
        Singleton {
            name: name.to_string(),
            age: age,
        }
    }
    fn instance(name: &str, age: u8) -> &'static mut Singleton {
        static mut INSTANCE: Option<Box<Singleton>> = None;
        unsafe {
            match INSTANCE {
                None => {
                    INSTANCE = Some(Box::new(Singleton::new(name, age)));
                }
                _ => (),
            }
            &mut *INSTANCE.as_mut().unwrap()
        }
    }
    // 其他方法
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u8 {
        self.age
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    fn set_age(&mut self, age: u8) {
        self.age = age;
    }
}

fn main() {
    let singleton1 = Singleton::instance("John", 30);
    singleton1.set_name("Peter");
    singleton1.set_age(35);
    println!("{} is {} years old.", singleton1.get_name(), singleton1.get_age());
}
