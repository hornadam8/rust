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
