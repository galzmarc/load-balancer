use std::net::SocketAddr;
use std::thread;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let addrs = [SocketAddr::from(([127, 0, 0, 1], 7878))];
    let listener = TcpListener::bind(&addrs[..]).expect("Failed to bind to address");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
fn handle_connection(mut client_stream: TcpStream) {
    let buf_reader = BufReader::new(&mut client_stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // Combine the HTTP request lines into a single string,
    // as backend server expects to receive the entire HTTP request formatted as such
    let http_request = http_request.join("\r\n") + "\r\n\r\n";

    // Forward the request to backend server
    let mut backend_stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    backend_stream.write_all(http_request.as_bytes()).unwrap();

    // Read the response from the backend server
    let mut response = Vec::new();
    backend_stream.read_to_end(&mut response).unwrap();

    // Send the response back to the client
    client_stream.write_all(&response).unwrap();
}
