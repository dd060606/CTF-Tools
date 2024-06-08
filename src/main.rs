use std::env;

use crate::client::start_client;
use crate::server::start_server;

mod client;
mod commands;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() { 1 => start_server(),
        3 => start_client(),
        _ => println!("Usage:\n  ctftools                  Start the CTF-Tools console\n  ctftools <ip> <port>      Start the CTF-Tools client")
    }
}
