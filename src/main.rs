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

    // TODO: (faqsarg - 20/05/2024) handle the Result<> from .lines() properly instead of unwrapping
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status = "HTTP/1.1 200 OK"; // lol best web server ever!!!
    let content = fs::read_to_string("ok.html").unwrap(); // TODO: (faqsarg - 20/05/2024) handle error
                                                          // properly
    let length = content.len();

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");

    // TODO: (faqsarg - 20/05/2024) proper error handling
    stream.write_all(response.as_bytes()).unwrap();
}
