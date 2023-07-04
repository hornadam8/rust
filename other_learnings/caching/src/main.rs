mod sized_lru;
mod timed_cache;

use std::{thread, time};
use std::time::SystemTime;
use crate::sized_lru::{fruit_stand, nth_fib, update_fruit_stand};
use crate::timed_cache::get_jwt;

#[tokio::main]
async fn main() {
    basic_lru_ops();
    lru_with_updates().await;
    timed_cache();
}

// calling an LRU cached function with size 2
fn basic_lru_ops() {
    // LRU cache testing, size 2 for simplicity

    println!("50 step fib\n");
    let start = SystemTime::now();
    let nth = nth_fib(50);
    let elapsed = start.elapsed().unwrap();
    let nth = nth.unwrap_or(0);
    println!("nth_fib: {nth:?}");
    println!("first call with 50 took: {elapsed:?}");

    println!("\n50 step fib\n");
    let start = SystemTime::now();
    let nth = nth_fib(50);
    let elapsed = start.elapsed().unwrap();
    let nth = nth.unwrap_or(0);
    println!("nth_fib: {nth:?}");
    println!("second call with 50 took: {elapsed:?}");

    println!("\n60 step fib\n");
    let start = SystemTime::now();
    let nth = nth_fib(60);
    let elapsed = start.elapsed().unwrap();
    let nth = nth.unwrap_or(0);
    println!("nth_fib: {nth:?}");
    println!("first call with 60 took: {elapsed:?}");

    println!("\n60 step fib\n");
    let start = SystemTime::now();
    let nth = nth_fib(60);
    let elapsed = start.elapsed().unwrap();
    let nth = nth.unwrap_or(0);
    println!("nth_fib: {nth:?}");
    println!("second call with 60 took: {elapsed:?}");

    println!("\n70 step fi\n");
    let start = SystemTime::now();
    let nth = nth_fib(70);
    let elapsed = start.elapsed().unwrap();
    let nth = nth.unwrap_or(0);
    println!("nth_fib: {nth:?}");
    println!("first call with 70 took: {elapsed:?}");

    println!("\n70 step fib\n");
    let start = SystemTime::now();
    let nth = nth_fib(70);
    let elapsed = start.elapsed().unwrap();
    let nth = nth.unwrap_or(0);
    println!("nth_fib: {nth:?}");
    println!("second call with 70 took: {elapsed:?}");

    println!("\n50 step fib\n");
    let start = SystemTime::now();
    let nth = nth_fib(50);
    let elapsed = start.elapsed().unwrap();
    let nth = nth.unwrap_or(0);
    println!("nth_fib: {nth:?}");
    println!("third call with 50 (out of cache) took: {elapsed:?}");

    println!("\n60 step fib\n");
    let start = SystemTime::now();
    let nth = nth_fib(60);
    let elapsed = start.elapsed().unwrap();
    let nth = nth.unwrap_or(0);
    println!("nth_fib: {nth:?}");
    println!("third call with 60 (out of cache) took: {elapsed:?}");

    println!("\n70 step fib\n");
    let start = SystemTime::now();
    let nth = nth_fib(70);
    let elapsed = start.elapsed().unwrap();
    let nth = nth.unwrap_or(0);
    println!("nth_fib: {nth:?}");
    println!("third call with 70 took: {elapsed:?}");
}

async fn lru_with_updates() {
    println!("\n\n\n\ngetting apples first time\n");
    let start = SystemTime::now();
    let price = fruit_stand("apple".to_string()).await;
    let elapsed = start.elapsed().unwrap();
    let price = price.unwrap_or(0.0);
    println!("price: {price:?}");
    println!("first call with apples took: {elapsed:?}");

    println!("\n\ngetting apples second time\n");
    let start = SystemTime::now();
    let price = fruit_stand("apple".to_string()).await;
    let elapsed = start.elapsed().unwrap();
    let price = price.unwrap_or(0.0);
    println!("price: {price:?}");
    println!("second call with apples took: {elapsed:?}");

    println!("\n\nupdating apples price to 0.15\n");
    let start = SystemTime::now();
    update_fruit_stand("apple".to_string(), 0.15).await;
    let elapsed = start.elapsed().unwrap();
    println!("update apples took: {elapsed:?}");

    println!("\n\ngetting apples third time, new price");
    let start = SystemTime::now();
    let price = fruit_stand("apple".to_string()).await;
    let elapsed = start.elapsed().unwrap();
    let price = price.unwrap_or(0.0);
    println!("price: {price:?}");
    println!("third call with apples took: {elapsed:?}");
}

fn timed_cache() {
    println!("\n\n\n\nfirst call for jwt\n");
    let start = SystemTime::now();
    let token = get_jwt();
    let elapsed = start.elapsed().unwrap();
    println!("first JWT call took: {elapsed:?}");
    println!("have token: {}", token);

    println!("\nsecond call for jwt");
    let start = SystemTime::now();
    let token = get_jwt();
    let elapsed = start.elapsed().unwrap();
    println!("second JWT call took: {elapsed:?}");
    println!("have token: {}", token);

    println!("\nthird call for jwt");
    let start = SystemTime::now();
    let token = get_jwt();
    let elapsed = start.elapsed().unwrap();
    println!("third JWT call took: {elapsed:?}");
    println!("have token: {}", token);

    println!("\nSleeping one second to let jwt expire");
    let wait = time::Duration::from_secs(1);
    thread::sleep(wait);

    println!("\nfourth call for jwt (expired)\n");
    let start = SystemTime::now();
    let token = get_jwt();
    let elapsed = start.elapsed().unwrap();
    println!("\nthird JWT call took: {elapsed:?}");
    println!("\nhave token: {}", token);
}