use std::net::TcpListener;

const IP: &str = "127.0.0.1";
const PORT: &str = "8080";

fn main() {
    let addr = format!("{}:{}", IP, PORT);

    // TODO: (faqsarg - 20/05/2024) handle error properly
    let lsnr = TcpListener::bind(addr).unwrap();

    for stream in lsnr.incoming() {
        // TODO: (faqsarg - 20/05/2024) avoid unwrap?
        let stream = stream.unwrap();

        println!("connection established");
    }
}
