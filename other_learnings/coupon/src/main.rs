use rustc_hash::FxHashMap; // non-cryptographic hasher
use std::time::{Duration, SystemTime};

fn main() {
    let (pieces, steps) = get_coupon(&mut vec![]); // getting steps and pieces in untimed solo run first
    let mut times: Vec<Duration> = Vec::with_capacity(10_000);
    let start = SystemTime::now();
    for _ in 1..10_001 {
        let (_,_) = get_coupon(&mut times);
    }
    let elapsed = start.elapsed().unwrap();
    times.sort();
    let fastest = times[0];
    let slowest = times[times.len()-1];
    let coupon = format_coupon(pieces);
    println!("took {} steps", steps);
    println!("Have coupon: {}", coupon);
    println!("fastest: {fastest:?}");
    println!("slowest: {slowest:?}");
    println!("total 10k {elapsed:?}");
}

fn format_coupon(res: Vec<i32>) -> String {
    let mut coupon = String::new();
    for v in res.iter() {
        coupon += format!("{} ", v).as_str();
    }
    return coupon.trim().to_string()
}

fn get_coupon(times: &mut Vec<Duration>) -> (Vec<i32>, i16) {
    let start = SystemTime::now();
    let mut count = 0;
    let mut sum_of_cubes: FxHashMap<i32, (i32,i32)> = FxHashMap::default();
    let mut coupon_pieces: Vec<i32> = Vec::with_capacity(16);
    for x  in 1..30 {                // 30 can be ruled out in this loop because y can't be 31
        for y in (&x + 1)..31 {
            count += 1;
            let x_pow = i32::pow(x, 3);
            let y_pow = i32::pow(y, 3);
            let sum: i32 = x_pow + y_pow;
            if sum_of_cubes.contains_key(&sum) {
                let (a,b) = sum_of_cubes
                    .remove(&sum)
                    .unwrap();
                coupon_pieces.push(a);
                coupon_pieces.push(b);
                coupon_pieces.push(x);
                coupon_pieces.push(y);
                if coupon_pieces.len() == 16 {
                    let elapsed = start.elapsed().unwrap();
                    times.push(elapsed);
                    return (coupon_pieces, count)
                }
            }
            sum_of_cubes.insert(x_pow + y_pow, (x,y));
        }
    }
    (coupon_pieces, count)
}