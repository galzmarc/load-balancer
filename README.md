# Rust Load Balancer

### Overview

This project is a simple load balancer implemented in Rust. It uses a round-robin algorithm to distribute incoming HTTP requests to a set of backend servers. The load balancer is multi-threaded, using a thread pool to handle incoming connections concurrently.

### Features

- Round-Robin Load Balancing: Distributes requests evenly across backend servers.
- Thread Pool: Handles multiple connections concurrently for better performance.
- Dynamic Backend Management: Allows adding and removing backend servers dynamically (work in progress).

### Code Structure

main.rs: The entry point of the application. Sets up the listener, thread pool, request queue, and backend servers.
lib.rs: Contains the implementation of the ThreadPool, Worker, Server, and RoundRobin structs, along with their associated methods.
Explanation of Key Parts

#### main.rs
- Sets up a TCP listener on the specified address.
- Initializes a thread pool and a request queue.
- Accepts incoming connections and adds them to the request queue.
- Spawns a thread to process the request queue and forward requests to backend servers using a round-robin algorithm.

#### lib.rs
- ThreadPool: Manages a pool of worker threads to handle incoming connections.
- Worker: Represents a worker thread in the thread pool.
- Server: Represents a backend server with an address.
- RoundRobin: Implements the round-robin algorithm to distribute requests evenly across backend servers.

### Future Enhancements

- Error Handling and Resilience: Implement logic to handle backend server failures gracefully.
- Logging and Monitoring: Add detailed logging and metrics collection.
- Asynchronous I/O: Switch to asynchronous I/O for better performance and scalability.
- Configuration Management: Allow dynamic updating of backend servers without restarting the load balancer.
- Security Features: Implement TLS/SSL termination and rate limiting.

### Contribution

Contributions are welcome! Please submit a pull request or open an issue to discuss any changes or enhancements.
