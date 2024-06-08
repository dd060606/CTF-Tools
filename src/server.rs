use std::{process, thread};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use colored::Colorize;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::commands::CommandHandler;
use crate::connections::Connections;

pub fn start_server() {
    print!("\x1B[2J\x1B[1;1H");
    println!("   ______ ______ ______    ______               __     ");
    println!("  / ____//_  __// ____/   /_  __/____   ____   / /_____");
    println!(" / /      / /  / /_ ______ / /  / __ \\ / __ \\ / // ___/");
    println!("/ /___   / /  / __//_____// /  / /_/ // /_/ // /(__  )");
    println!("\\____/  /_/  /_/         /_/   \\____/ \\____//_//____/ \n");
    let connections = Arc::new(Mutex::new(Connections::new()));
    start_tcp_server(&connections);

    thread::sleep(Duration::from_millis(500));

    let command_handler = CommandHandler::new(&connections);

    let mut rl = DefaultEditor::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });

    loop {
        let readline = rl.readline("# ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                if line.is_empty() {
                    continue;
                }
                let cmd_res = command_handler.handle_command(line);
                match cmd_res {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("[{}] {}", "-".red(), err)
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn start_tcp_server(connections: &Arc<Mutex<Connections>>) {
    let connections = Arc::clone(connections);
    let _ = thread::spawn(move || {
        let mut port: u16 = 8888;
        //Bind a TCP listener on an available port
        let listener = loop {
            match TcpListener::bind(format!("0.0.0.0:{}", port)) {
                Ok(listener) => break listener,
                Err(e) => {
                    port += 1;
                    if port > 9500 {
                        panic!("Error while starting tcp server: {}", e);
                    }
                }
            }
        };
        println!("[{}] Server listening on port {}", "+".bright_green(), port);
        // Counter to assign unique IDs to connections
        let mut next_id: u16 = 1;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mut connections = connections.lock().unwrap();
                    println!(
                        "[{}] New connection: {} {}",
                        "+".bright_green(),
                        next_id,
                        stream.peer_addr().unwrap()
                    );
                    connections.add_connection(next_id, stream);
                    next_id += 1;
                }
                Err(e) => {
                    eprintln!("[{}] Connection failed: {}", "-".red(), e);
                }
            }
        }
        //Close the server
        drop(listener);
    });
}
