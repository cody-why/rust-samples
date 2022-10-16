use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread, time::Duration};


fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    
    thread::sleep(Duration::from_secs(5));

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, contents) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", fs::read_to_string("hello.html").unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND", String::from("Not Found"))
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}