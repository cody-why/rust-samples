### volo get start
* https://www.cloudwego.io/zh/docs/volo/volo-thrift/getting-started/

### init idl
* volo init volo-example idl/volo_example.thrift
### or add idl file
* volo idl add idl/volo_example2.thrift

### build
* cargo update 
* cargo build
* cargo run --bin server

### 升级到0.2
* 更新volo-cli
cargo install volo-cli
* https://github.com/cloudwego/cloudwego.github.io/commit/7c07eb88df1a3823d511bf7641f0cc7cbf06fe5a#diff-7ed3fd4789569fd0595b31568bfc63042e4e96915bf3cffa52e21da8ac903282