use std::{
    fs,
    io::prelude::*,
    net::{
        TcpListener,
        TcpStream
    }
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection (mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_home = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get_home) {
        let hello_page = fs::read_to_string("hello.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            hello_page.len(),
            hello_page
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let not_found_page = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\n\r\n{}",
            not_found_page
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

}