use std::fmt::Debug;

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
// Box<T> 把它当成一个引用即可，只不过它包裹的值会被强制分配在堆上
//dyn = dynamic 动态分发
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
#[allow(dead_code)]
/// 作为参数1
fn noise1(a: &impl Animal) {
    a.noise();
}
#[allow(dead_code)]
/// 作为参数2
fn noise2<T: Animal>(a: T) {
    a.noise();
}
#[allow(dead_code)]
/// 作为参数3
fn noise3<T>(a: T)
where
    T: Animal + Debug,
{
    a.noise();
}
