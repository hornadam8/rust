use std::time::SystemTime;

fn main() {
    let mut count = 0;
    let start = SystemTime::now();
    let mut i = 1;
    loop {
        if has_no_subints_divisible_by_9(i) {
            count += 1;
        }
        i += 1;
    }
    let elapsed = start.elapsed().unwrap();
    println!("Finished in {elapsed:?}");
    println!("{count} numbers have no integer substrings divisible by 9");
}

fn has_no_subints_divisible_by_9(n: i32) -> bool {
    let s_int = format!("{}", n);
    for i in 0..s_int.len() {
        for j in i..s_int.len() {
            let sub = s_int.get(i..=j).unwrap().parse::<i32>().unwrap();
            if sub % 9 == 0 {
                return false;
            }
        }
    }
    true
}
