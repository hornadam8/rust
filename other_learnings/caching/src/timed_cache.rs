use std::collections::HashMap;
use std::{thread, time};
use cached::proc_macro::cached;
use cached::{
    Cached, SizedCache
};

// a timed cache, currently I like this for jwt but I'm sure there are other good uses

#[cached(time = 1)] // cache jwt for 1 second
pub fn get_jwt() -> String {
    println!("JWT expired, generating new one");

    println!("simulating jwt generation call time");
    let jwt_time = time::Duration::from_millis(66);
    thread::sleep(jwt_time);

    return String::from("jwt-token")
}