use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fmt::format
};


fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

fn send_page (status_line: &str, page_name: &str, mut stream: TcpStream) {
    let content = fs::read_to_string(page_name).unwrap();
    let length = content.len();

    let res = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(res.as_bytes()).unwrap();
}

fn handle_connection (mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    if req_line == "GET / HTTP/1.1" {
        send_page("HTTP/1.1 200 OK", "index.html", stream);
    } else {
        send_page("HTTP/1.1 404 Not Found", "404.html", stream);
    }
    
}