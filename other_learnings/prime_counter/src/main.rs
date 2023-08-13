use std::env::args;
use std::time::SystemTime;


fn main() {
    let args: Vec<String> = args().collect();
    let n: u32 = args.get(1)
        .unwrap_or(&"".to_string())
        .parse()
        .unwrap_or(100);

    let start = SystemTime::now();
    let count = count_primes(n);
    let elapsed = start.elapsed().unwrap();

    println!("primes under {n}: {count}");
    println!("took {elapsed:?}");
}
const WHEEL: [u32; 8] = [7, 11, 13, 17, 19, 23, 29, 31];

fn is_prime(n: u32) -> bool {
    if n == 2 || n == 3 || n == 5 {
        return true
    }
    if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 {
        return false
    }
    let sqrt_n = (n as f32).sqrt() as u32;
    for i in (0..sqrt_n).step_by(30) {
        'wheel: for j in WHEEL {
            if j > sqrt_n {
                break 'wheel;
            }
            if n % (j + i) == 0 {
                return false
            }
        }
    }
    return true
}

fn count_primes(n: u32) -> u32 {
    let mut count = 1;
    for i in (3..n).step_by(2) {
        if is_prime(i) {
            count += 1;
        }
    }
    count
}