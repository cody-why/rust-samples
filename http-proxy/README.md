## build
cargo build --release --target x86_64-pc-windows-gnu

## run
http_proxy 3082


// To try this example:
// 1. config http_proxy in command line
//    $ export http_proxy=http://127.0.0.1:3082
//    $ export https_proxy=http://127.0.0.1:3082
// 2. send requests
//    $ curl -i https://www.some_domain.com/