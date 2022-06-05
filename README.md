# Rust-Basic-PortScanner
A basic TCP connect port scanner built in Rust.

## Description
Implements a basic TCP connect port scanner in Rust. Has various options similar to nmap that allow scanning common ports, ranges of ports, and all ports.

## Usage
Valid options:
- port_scanner [ip]
- port_scanner [ip] -p [port]
- port_scanner [ip] -r [port] [port]
- port_scanner [ip] -a

## Technologies Used
Rust

## Improvements
- Add closures to reduce repetition.
- Add concurrency to speed up slow scanning.

## References
[Fast Port Scanner](https://kerkour.com/rust-fast-port-scanner)\
[nmap](https://nmap.org/book/man-port-scanning-techniques.html)

## License

This project is licensed under the  MIT License - see the LICENSE file for details.
