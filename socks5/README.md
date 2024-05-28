## build
cargo build -r --target x86_64-pc-windows-gnu
cargo build -r --target x86_64-unknown-linux-gnu


## run
socks5 config.yaml

## curl
curl --socks5 127.0.0.1:3080 baidu.com

## 使用用户密码
curl -U user:pasa* --socks5 127.0.0.1:3080 baidu.com


cargo fix --allow-dirty