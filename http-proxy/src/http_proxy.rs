#![deny(warnings)]



use bytes::Bytes;
use http_body::{Empty, Full};
use http_body::combinators::BoxBody;
// use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::HttpBody;
use hyper::client::conn::Builder;
use hyper::server::conn::Http;
// use hyper::client::conn::http1::Builder;
// use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::upgrade::Upgraded;
use hyper::{Method, Request, Response, http};
use tokio::net::{TcpListener, TcpStream};

/// 一个声明宏,输出日志时,自动添加target: "proxy"

macro_rules! info {
    ($($arg:tt)+) => {
        log::info!(target: "proxy", $($arg)+)
    };
}

macro_rules! debug {
    ($($arg:tt)+) => {
        log::debug!(target: "proxy", $($arg)+)
    };
}

// To try this example:
// 1. config http_proxy in command line
//    $ export http_proxy=http://127.0.0.1:3082
//    $ export https_proxy=http://127.0.0.1:3082
// 2. send requests
//    $ curl -i https://www.some_domain.com/
pub async fn run(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let addr:SocketAddr = addr.parse()?;
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            // if let Err(err) = http1::Builder::new()
            //     .preserve_header_case(true)
            //     .title_case_headers(true)
            //     .serve_connection(stream, service_fn(proxy))
            //     .with_upgrades()
            //     .await
            if let Err(err) = Http::new()
                .http1_preserve_header_case(true)
                .http1_title_case_headers(true)
                .serve_connection(stream, service_fn(proxy))
                .with_upgrades()
                .await
            {
                info!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn proxy(
    req: Request<hyper::Body>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    info!( "req: {:?} method: {:?}", req.uri(), req.method());
    // only support local host
    if req.uri().host().unwrap_or_default() != "127.0.0.1" {
        info!("not support host: {:?}", req.uri());
        return Ok(Response::new(full("not support")));
    }
    if Method::CONNECT == req.method() {
        // Received an HTTP request like:
        // ```
        // CONNECT www.domain.com:443 HTTP/1.1
        // Host: www.domain.com:443
        // Proxy-Connection: Keep-Alive
        // ```
        //
        // When HTTP method is CONNECT we should return an empty body
        // then we can eventually upgrade the connection and talk a new protocol.
        //
        // Note: only after client received an empty body with STATUS_OK can the
        // connection be upgraded, so we can't return a response inside
        // `on_upgrade` future.
        if let Some(addr) = host_addr(req.uri()) {
            tokio::task::spawn(async move {
                match hyper::upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, addr).await {
                            info!("server io error: {}", e);
                        };
                    }
                    Err(e) => info!("upgrade error: {}", e),
                }
            });

            Ok(Response::new(empty()))
        } else {
            info!("CONNECT host is not socket addr: {:?}", req.uri());
            let mut resp = Response::new(full("CONNECT must be to a socket address"));
            *resp.status_mut() = http::StatusCode::BAD_REQUEST;

            Ok(resp)
        }
    } else {
        let host = req.uri().host().expect("uri has no host");
        let port = req.uri().port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(addr).await.unwrap();

        // let (mut sender, conn) = Builder::new()
        //     .preserve_header_case(true)
        //     .title_case_headers(true)
        //     .handshake(stream)
        //     .await?;
        let (mut sender, conn) = Builder::new()
            .http1_preserve_header_case(true)
            .http1_title_case_headers(true)
            .handshake(stream)
            .await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                info!("Connection failed: {:?}", err);
            }
        });

        let resp = sender.send_request(req).await?;
        Ok(resp.map(|b| b.boxed()))
    }
}

fn host_addr(uri: &http::Uri) -> Option<String> {
    uri.authority().map(|auth| auth.to_string())
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

// Create a TCP connection to host:port, build a tunnel between the connection and
// the upgraded connection
async fn tunnel(mut upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    // Connect to remote server
    let mut server = TcpStream::connect(addr).await?;

    // Proxying data
    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    // Print message when done
    debug!(
        "client wrote {} bytes and received {} bytes",
        from_client, from_server
    );

    Ok(())
}