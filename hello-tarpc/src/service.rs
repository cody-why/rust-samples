/*
 * @Author: plucky
 * @Date: 2023-07-18 16:28:29
 * @LastEditTime: 2023-07-18 17:28:39
 */

#[tarpc::service]
trait World {
    async fn hello(name: String, msg: String) -> String;
}


#[derive(Clone)]
struct HelloServer(String);

#[tarpc::server]
impl World for HelloServer {
    async fn hello(self, _: tarpc::context::Context, name: String, msg: String) -> String {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        // format!("Hello, {name}! You are connected from {}: {msg}", self.0)
        let _ = (name, msg);
        "Hello".to_string()
    }
}