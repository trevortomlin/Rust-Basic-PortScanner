/*
    Simple port scanner like nmap
    Author: Trevor Tomlin
    Created: June 5, 2022
*/

// References
// https://kerkour.com/rust-fast-port-scanner
// https://nmap.org/book/man-port-scanning-techniques.html

use std::net::TcpStream;

#[derive(Debug)]
enum PortStatus {

    OPEN,
    CLOSED

}

pub const MOST_COMMON_PORTS_1002: &[u16] = &[
    5601, 9300, 80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111,
    995, 993, 5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179,
    1026, 2000, 8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666, 646,
    5000, 5631, 631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513, 990, 5357,
    427, 49156, 543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070, 5190, 3000, 5432,
    1900, 3986, 13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899, 9100, 119, 37, 1000,
    3001, 5001, 82, 10010, 1030, 9090, 2107, 1024, 2103, 6004, 1801, 5050, 19, 8031, 1041, 255
];

fn main() {

    let args: Vec<String> = std::env::args().collect();

    // Valid options
    // port_scanner [ip]
    // port_scanner [ip] -p [port]
    // port_scanner [ip] -r [port] [port]
    // port_scanner [ip] -a

    match args.len() {

        2 => scan_common_ports(&args[1]),
        3 => scan_all_ports(&args[1]),
        4 => println!("Scanning port {}\n{} is {:?}", &args[3], &args[3], scan_port(&args[1], &args[3])),
        5 => scan_range(&args[1], &args[3], &args[4]),
        _ => print_usage(),

    }

}

fn print_usage() {

    println!("Missing arguments.");
    println!("Usage:");
    println!("port_scanner [ip]");
    println!("port_scanner [ip] -p [port]");
    println!("port_scanner [ip] -r [port] [port]");
    println!("port_scanner [ip] -a");

}

// TCP connect scan
fn scan_port(ip: &str, port: &str) -> PortStatus{

    let addrs = format!("{}:{}", ip, port);

    let mut port_status = PortStatus::OPEN;

    if let Ok(stream) = TcpStream::connect(addrs) {
        port_status = PortStatus::OPEN;
    } else {
        port_status = PortStatus::CLOSED;
    }

    return port_status;

}

// Scans top ports
fn scan_common_ports(ip: &str) {

    println!("Scanning most common ports");

    for i in 0..MOST_COMMON_PORTS_1002.len() {
        
        let port = MOST_COMMON_PORTS_1002[i].to_string();
        println!("PORT {} is {:?}", port, scan_port(ip, &port));

    }

}
// Scans all ports 0-65535
fn scan_all_ports(ip: &str) {

    println!("Scanning all ports");

    for i in 0..65535 {
        
        let port = i.to_string();
        println!("PORT {} is {:?}", port, scan_port(ip, &port));

    }

}
// Scan ports between two numbers
fn scan_range(ip: &str, port: &str, port2: &str) {

    println!("Scanning from port {} to port {}", port, port2);

    for i in port.parse::<i32>().unwrap()..port2.parse::<i32>().unwrap() {
        
        let port = i.to_string();
        println!("PORT {} is {:?}", port, scan_port(ip, &port));

    }

}