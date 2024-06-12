#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => (
        println!("[{}] {}","+".bright_green(), format!($($arg)*));
    )
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        eprintln!("[{}] {}","-".red(), format!($($arg)*));
    )
}
