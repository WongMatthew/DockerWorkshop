use std::net::{Shutdown, TcpListener};
use std::thread;
use std::io::Write;

const RESPONSE: &'static [u8] = b"HTTP/1.1 200 OK\r
Content-Type: text/html; charset=UTF-8\r\n\r
<!DOCTYPE html><html><head><title>Hello World Rust</title></head>
<body><h1>Hello World, from Rust!</h1></body></html>\r";

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8001").unwrap();
    println!("Rust web server running at: http://0.0.0.0:8001");

    for stream in listener.incoming() {
        thread::spawn(move || {
            let mut stream = stream.unwrap();
            match stream.write(RESPONSE) {
                Ok(_) => println!("Response sent successfully!"),
                Err(e) => println!("Failed sending response: {}!", e),
            }
            stream.shutdown(Shutdown::Write).unwrap();
        });
    }
}