use std::collections::HashMap;

fn main() {

    let s = String::from("hello");
    takes_ownership(s);
    // s no longer in scope

    let x = 5;
    shallow_copy(x);
    println!("Still have x: {}", x);

    let t = true;
    shallow_or_deep_bool(t);
    println!("Still have t: {}", t);

    let y: i64 = 1920391023931;
    shallow_or_deep_i64(y);
    println!("Still have y: {}", y);

    let h = HashMap::from([
        ("Foo", "Bar"),
        ("baz", "qux"),
        ("quux", "garply"),
    ]);

    shallow_or_deep_hash(h);
    // h is no longer in scope
}

fn takes_ownership(a_string: String) {
    println!("{}", a_string);
}

fn shallow_copy(an_i32: i32){
    println!("{}", an_i32);
}

fn shallow_or_deep_bool(a_bool: bool) {
    println!("{}", a_bool);
}

fn shallow_or_deep_i64(an_i64: i64) {
    println!("{}", an_i64);
}

fn shallow_or_deep_hash(a_hash: HashMap<&str,&str>) {
    for (key, value) in &a_hash {
        println!("{}: {}", key, value);
    }
}
