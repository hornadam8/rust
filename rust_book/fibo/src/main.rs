use std::time::UNIX_EPOCH;

fn main() {
    /*
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
     */
    let mut n = 2;
    let mut max_steps_temp = 0;
    let mut max_steps = 0;
    let mut max_n_temp = 0;
    let mut max_n = 0;
    loop {
        let steps = collatz(n);
        if steps > max_steps {
            max_steps = steps;
            max_n = n;
        }
        if steps > max_steps_temp {
            max_steps_temp = steps;
            max_n_temp = n;
        }
        if n % 1_000_000 == 0 {
            println!("\n\ncollatz reached {n}");
            println!("longest path over last 1M:\n{max_n_temp} took {max_steps_temp} steps");
            println!("longest path ever:\n{max_n} took {max_steps} steps");
            max_steps_temp = 0;
        }
        n += 1;
    }
    //collatz(113383);
}

fn fibonacci(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn collatz(n: usize) -> i32 {
    let mut steps = 0;
    //println!("collatz for {n}");
    let mut n = n;
    while (n != 1) {
        steps += 1;
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = (n * 3) + 1;
        }
    }
    //println!("Found in {steps} steps");
    steps
}
