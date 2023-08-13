use std::{env, thread, time};

pub fn sleep(s: u64) {
    if env::var("FAST").unwrap_or("false".to_string()) == "true" {
        return;
    }
    thread::sleep(time::Duration::from_secs(s));
}

pub fn print_if(s: &str) {
    if env::var("PRINT").unwrap_or("true".to_string()) == "false" {
        return;
    }
    println!("{}", s);
}

pub fn clear_terminal() {
    // This is the ANSI escape code to clear the terminal
    print!("\x1B[2J\x1B[1;1H");
    print!("\x1B[1;1H");
}
