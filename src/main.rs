use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const IP: &str = "127.0.0.1";
const PORT: &str = "8080";

fn main() {
    let addr = format!("{}:{}", IP, PORT);

    // TODO: (faqsarg - 20/05/2024) handle error properly
    let lsnr = TcpListener::bind(addr).unwrap();

    for stream in lsnr.incoming() {
        // TODO: (faqsarg - 20/05/2024) avoid unwrap?
        let stream = stream.unwrap();

        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if req_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "ok.html")
    } else {
        ("HTTP/1.1 400 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    // TODO: (faqsarg - 20/05/2024) proper error handling
    stream.write_all(response.as_bytes()).unwrap();
}
