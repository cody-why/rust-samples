## build
cargo build --release --target x86_64-pc-windows-gnu

## run
socks5proxy -l 127.0.0.1:3080

## curl
curl --socks5 127.0.0.1:3080 baidu.com

## 使用用户密码
curl -U user:pasa* --socks5 127.0.0.1:3080 baidu.com
 