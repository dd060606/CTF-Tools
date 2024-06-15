use std::io::{Read, Result, stdin, stdout, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self, JoinHandle};

use colored::Colorize;
use encoding_rs::UTF_8;

#[cfg(unix)]
use signal_hook::{consts, iterator::Signals};

#[cfg(unix)]
use crate::shell::setup_fd;

// It will complain on unix systems without this lint rule.
#[allow(dead_code)]
fn print_feature_not_supported() {
    error!("This feature is not supported on your platform");
}

fn pipe_thread<R, W>(mut r: R, mut w: W, tx: Sender<()>) -> JoinHandle<()>
where
    R: Read + Send + 'static,
    W: Write + Send + 'static,
{
    thread::spawn(move || {
        let mut buffer = [0; 1024];

        loop {
            match r.read(&mut buffer) {
                Ok(0) => {
                    let _ = tx.send(());
                    break;
                }
                Ok(len) => {
                    let (decoded, ..) = UTF_8.decode(&buffer[..len]);

                    if let Err(err) = w.write_all(decoded.as_bytes()) {
                        error!("{}", err);
                        break;
                    }
                }
                Err(err) => {
                    error!("{}", err);
                    break;
                }
            }

            if let Err(err) = w.flush() {
                error!("{}", err);
                break;
            }
        }
    })
}

fn listen_tcp_normal(stream: TcpStream) -> Result<()> {
    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();

    let (stdin_thread, stdout_thread) = (
        pipe_thread(stdin(), stream.try_clone()?, tx.clone()),
        pipe_thread(stream, stdout(), tx),
    );

    success!("Connection Received");

    // Wait for either thread to finish
    rx.recv().ok();
    stdin_thread.join().unwrap();
    stdout_thread.join().unwrap();

    Ok(())
}

// Listen on given host and port
pub fn listen(port: u16) -> Result<()> {
    //Bind a TCP listener on an available port
    let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)) {
        Ok(listener) => listener,
        Err(e) => {
            error!("Error while listening on port {}: {}", port, e);
            return Ok(());
        }
    };

    success!("Listening on 0.0.0.0:{}", port);
    let (stream, _) = listener.accept()?;
    #[cfg(unix)]
    {
        Signals::new(&[consts::SIGINT])?;
        setup_fd()?;
        listen_tcp_normal(stream)?;
    }
    #[cfg(windows)]
    listen_tcp_normal(stream)?;

    Ok(())
}
