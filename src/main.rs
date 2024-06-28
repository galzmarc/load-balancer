use std::collections::VecDeque;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

use load_balancer::{RoundRobin, Server, ThreadPool};

type Queue = Arc<Mutex<VecDeque<TcpStream>>>;

fn main() {
    let addrs = [SocketAddr::from(([127, 0, 0, 1], 7878))];
    let listener = TcpListener::bind(&addrs[..]).expect("Failed to bind to address");
    let pool = ThreadPool::new(4);
    let request_queue: Queue = Arc::new(Mutex::new(VecDeque::new()));

    let backend_servers = vec![
        Server::new(SocketAddr::from(([127, 0, 0, 1], 8080))),
        Server::new(SocketAddr::from(([127, 0, 0, 1], 8081))),
    ];

    let round_robin = Arc::new(Mutex::new(RoundRobin::new()));
    for server in backend_servers {
        round_robin.lock().unwrap().insert_server(server);
    }

    let processing_queue = Arc::clone(&request_queue);
    let processing_round_robin = Arc::clone(&round_robin);
    let processing_pool = ThreadPool::new(4);
    thread::spawn(move || loop {
        let mut queue = processing_queue.lock().unwrap();

        while let Some(stream) = queue.pop_front() {
            let round_robin = Arc::clone(&processing_round_robin);
            processing_pool.execute(move || handle_connection(stream, round_robin));
        }

        drop(queue); // Explicitly dropping the lock

        // Sleep to prevent tight loop when queue is empty
        thread::sleep(Duration::from_millis(100));
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let queue_clone = Arc::clone(&request_queue);
                pool.execute(move || {
                    let mut queue = queue_clone.lock().unwrap();
                    queue.push_back(stream);
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }

    // Prevent the main thread from exiting
    loop {
        thread::park();
    }
}

fn handle_connection(mut client_stream: TcpStream, round_robin: Arc<Mutex<RoundRobin>>) {
    let buf_reader = BufReader::new(&mut client_stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // Combine the HTTP request lines into a single string,
    // as backend server expects to receive the entire HTTP request formatted as such
    let http_request = http_request.join("\r\n") + "\r\n\r\n";

    // Round Robin
    let backend_server = {
        let mut rr = round_robin.lock().unwrap();
        rr.next()
    };

    // Forward the request to backend server
    if let Some(backend_server) = backend_server {
        let addr = backend_server.addr;
        let mut backend_stream = TcpStream::connect(addr).unwrap();
        backend_stream.write_all(http_request.as_bytes()).unwrap();

        // Read the response from the backend server
        let mut response = Vec::new();
        backend_stream.read_to_end(&mut response).unwrap();

        // Send the response back to the client
        client_stream.write_all(&response).unwrap();
    } else {
        eprintln!("No backend server available");
    };
}
