pub use listener::listen;
#[cfg(unix)]
pub use termios_handler::setup_fd;

#[cfg(unix)]
pub mod unixshell;

#[cfg(unix)]
pub mod termios_handler;
mod listener;

#[cfg(windows)]
pub mod winshell;
