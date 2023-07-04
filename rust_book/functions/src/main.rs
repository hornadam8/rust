use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let input: i64 = input
        .trim()
        .parse()
        .expect("Not a number");

    let steps = collatz(input);

    println!("{steps}");
}

fn collatz(n: i64) -> i64 {
    let mut x = n;
    let mut steps = 0;
    while x != 1 {
        steps += 1;
        if x % 2 == 0 {
            x /= 2;
        } else {
            x = x * 3 + 1;
        }
    }
    steps
}