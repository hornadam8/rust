fn main() {
    let x: Option<i32> = Some(6);
    let y: Option<i32> = None;
    dbg!(x.is_some());
    dbg!(y.is_some());
    dbg!(x.is_none());
    dbg!(y.is_none());
    dbg!(x.is_some_and(|x| x > 2));
    dbg!(x.is_some_and(|x| x > 10));
    match x {
        Some(n) => add_2(n),
        None => println!("no value"),
    };
    match y {
        Some(n) => add_2(n),
        None => println!("no value"),
    };
}

fn add_2(n: i32) {
    println!("Inside add_2 where {} + 2 = {}", n, n+2);
}