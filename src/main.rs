/*
    Simple port scanner like nmap
    Author: Trevor Tomlin
    Created: June 5, 2022
*/

//use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {

        println!("Missing arguments.");
        println!("Usage:");
        println!("port_scanner [ip] [options]");
        //std::process::exit(1);
        return;

    }
    
    // Get ip from args
    let ip = &args[1];

}
