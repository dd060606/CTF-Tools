extern crate core;

use std::env;

use crate::client::start_client;
use crate::server::start_server;

mod client;
mod commands;
mod connections;

mod server;
#[macro_use]
mod macros;
mod files;
mod payloads;
mod shell;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() { 1 => start_server(),
        3 => start_client(args[1].clone(), args[2].clone()),
        _ => println!("Usage:\n  ctftools                  Start the CTF-Tools console\n  ctftools <ip> <port>      Start the CTF-Tools client")
    }
}
