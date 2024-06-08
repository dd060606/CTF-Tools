use std::process;

use colored::Colorize;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::commands::CommandHandler;

pub fn start_server() {
    print!("\x1B[2J\x1B[1;1H");
    println!("   ______ ______ ______    ______               __     ");
    println!("  / ____//_  __// ____/   /_  __/____   ____   / /_____");
    println!(" / /      / /  / /_ ______ / /  / __ \\ / __ \\ / // ___/");
    println!("/ /___   / /  / __//_____// /  / /_/ // /_/ // /(__  )");
    println!("\\____/  /_/  /_/         /_/   \\____/ \\____//_//____/ ");
    println!("\n{}", "Welcome to CTF-Tools !".bold().yellow());

    let mut rl = DefaultEditor::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    let command_handler = CommandHandler::new();
    loop {
        let readline = rl.readline("# ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
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
