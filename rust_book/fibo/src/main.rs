use std::time::UNIX_EPOCH;


fn main() {
    let start = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    let num: i32 = 42;
    fibonacci(num);
    let end = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    println!("Took {}ms", end - start);
}

fn fibonacci(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

