use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("collatz.dot").unwrap();
    file.write_all(b"digraph A {\n").unwrap();
    let mut i = 2;
    let mut seen: HashMap<usize, bool> = HashMap::default();
    while i < 100_000 {
        collatz(i, &mut file, &mut seen);
        i += 1;
    }
    file.write_all(b"}").unwrap();
}

fn collatz(n: usize, file: &mut File, seen: &mut HashMap<usize, bool>) {
    let mut n = n;
    while n != 1 {
        let new;
        if n % 2 == 0 {
            new = n / 2;
        } else {
            new = (n * 3) + 1;
        }
        if seen.get(&n).is_none() {
            let s = format!("{n} -> {new}\n");
            file.write_all(&s.into_bytes()).unwrap();
            seen.insert(n, true);
        } else {
            break;
        }
        n = new;
    }
}
