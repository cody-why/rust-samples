
// 定义一个结构
// 可以打印输出
#[derive(Debug)]
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}

fn teststu(){
    // 实例
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013
    };

    println!("{:?}",runoob);

    // 用变量名直接赋值
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");

    let runoob = Site {
        domain,  // 等同于 domain : domain,
        name,    // 等同于 name : name,
        nation: String::from("China"),
        found: 2013
    };

    println!("{:?}",runoob);
    
    //大部分属性和现存的一样,个别不一样
    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob 
        //后面没有逗号
    };

    println!("{:?}",site);

    //元组结构体,固定的类型格式
    struct Color(u8, u8, u8);
    let black = Color(0, 0, 0);
    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    
}

struct Rectangle {
    width: u32,
    height: u32,
}

//结构方法
impl Rectangle {
    //第一个参数必须是 &self
    //实例函数
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //结构体关联函数,等于静态函数,调用方法Rectangle::create()
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}


//测试怎么调用结构的方法
fn teststu2() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1's area is {}", rect1.area());
}


// `Pair` 持有两个分配在堆上的整数
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 该方法会拿走调用者的所有权
    // `self` 是 `self: Self` 的语法糖
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` 和 `second` 在这里超出作用域并被释放
    }
}
