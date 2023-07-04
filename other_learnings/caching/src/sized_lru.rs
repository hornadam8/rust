use std::collections::HashMap;
use std::{thread, time};
use cached::proc_macro::cached;
use cached::{
    Cached, SizedCache
};

#[cached(
result = true,                               //  so we don't cache errors
type = "SizedCache<usize, usize>",
create = "{ SizedCache::with_size(2) }"
)]
pub fn nth_fib(x: usize) -> Result<usize, String> {
    println!("INSIDE nth_fib, CACHE MISS");
    let mut last: usize;
    let mut cur: usize = 1;
    let mut next: usize = 1;
    let mut count: usize = 1;
    while count < x {
        last = cur;
        cur = next;
        next = cur + last;
        count += 1;
    }
    return Ok(cur)
}


// a cache that can be accessed and updated
// this uses a hashmap but really this would usually be a DB query

#[cached(
name = "FRUIT_CACHE",                        // cache is named so we can access its mutex elsewhere
result = true,                               //  so we don't cache errors
type = "SizedCache<String, f64>",
create = "{ SizedCache::with_size(2) }"
)]
pub async fn fruit_stand(fruit: String) -> Result<f64, String> {
    println!("INSIDE fruit_stand, CACHE MISS");

    println!("simulating database response time");
    let ddb_time = time::Duration::from_millis(130);
    thread::sleep(ddb_time);

    let prices = HashMap::from([
        ("apple".to_string(), 0.25),
        ("banana".to_string(), 0.3),
        ("orange".to_string(), 0.75)
    ]);
    return match prices.get(fruit.as_str()) {
        Some(price) => Ok(price.clone()),
        None => Err(format!("no price for {}", &fruit))
    }
}

pub async fn update_fruit_stand(fruit: String, price: f64) {
    {
        let mut cache = FRUIT_CACHE.lock().await;
        match cache.cache_get_mut(fruit.as_str()) {
            Some(old_price) => {
                *old_price = price;
                println!("{}", format!("Updated record in cache for fruit: {}", &fruit));
            },
            None => {
                println!("{}", format!("No record in cache for fruit: {}",
                                       &fruit));
            }
        }
        println!("{}", format!("{} records in cache",
                    cache.cache_size()))
    }; // mutex lock is dropped here and thus unlocks
}