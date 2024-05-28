/*
 * @Author: plucky
 * @Date: 2023-05-24 22:04:25
 * @LastEditTime: 2023-05-24 22:07:12
 * @Description: 
 */
// 适配器模式

//客户端代码需要使用的接口
trait Target {
    fn request(&self) -> String;
}
 struct Adaptee {
    adaptee_field: String,
}
 impl Adaptee {
    fn specific_request(&self) -> String {
        format!("Adaptee: {}", self.adaptee_field)
    }
}
 struct Adapter {
    adaptee: Adaptee,
}
 impl Target for Adapter {
    fn request(&self) -> String {
        self.adaptee.specific_request()
    }
}
 fn main() {
    let adaptee = Adaptee {
        adaptee_field: String::from("example adaptee field"),
    };
    let adapter = Adapter { adaptee: adaptee };
    let result = adapter.request();
    println!("{}", result);
}

// 在这个例子中，我们定义了一个 Target trait 来表示客户端代码需要使用的接口。然后我们定义了一个 Adaptee 结构体，它具有客户端不能直接使用的 specific_request 方法。 
// 接着我们定义了一个 Adapter 结构体，它实现了 Target trait 并包装了一个 Adaptee 结构体的实例。Adapter 的 request 方法通过调用 Adaptee 的 specific_request 方法来实现。 
// 最后，我们创建了一个 Adaptee 结构体和一个 Adapter 结构体来包装 Adaptee 实例。然后我们调用 Adapter 的 request 方法，该方法在内部调用 Adaptee 的 specific_request 方法并返回结果。 
// 这使得客户端代码可以使用 Adapter 的 request 方法，该方法在内部调用 Adaptee 的 specific_request 方法，而不必了解 Adaptee 的实现细节。这展示了适配器模式如何通过包装一个对象来适配不兼容的接口。