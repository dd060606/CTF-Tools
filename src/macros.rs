#[macro_export]
macro_rules! success {

    ($($arg:tt)*) => (
        use colored::Colorize;
        println!("[{}] {}","+".bright_green(), format!($($arg)*));
    )
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        use colored::Colorize;
        eprintln!("[{}] {}","-".red(), format!($($arg)*));
    )
}
