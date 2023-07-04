use std::fmt;

mod other_thing;

fn main() {
    let change: Vec<other_thing::Coin> = vec![
        other_thing::Coin::Penny,
        other_thing::Coin::Quarter(other_thing::UsState::Florida),
        other_thing::Coin::Nickle,
        other_thing::Coin::Dime,
        other_thing::Coin::Quarter(other_thing::UsState::Maryland),
        other_thing::Coin::Dime,
    ];
    let quarters: Vec<other_thing::Coin> = vec![
        other_thing::Coin::Quarter(other_thing::UsState::Colorado),
        other_thing::Coin::Quarter(other_thing::UsState::Arizona),
        other_thing::Coin::Quarter(other_thing::UsState::Kansas),
        other_thing::Coin::Quarter(other_thing::UsState::NewMexico),
        other_thing::Coin::Quarter(other_thing::UsState::Hawaii),
    ];
    count_quarters(quarters);
    //count_quarters(change);
    let change_val = count_change(change);
    println!("Have {} cents", change_val);

    let five = Some(5);
    dbg!(five);
    let six = plus_one(five);
    dbg!(six);
    let none = plus_one(None);
    dbg!(none);
}





fn count_change(coins: Vec<other_thing::Coin>) -> i32{
    let mut total: i32 = 0;
    for coin in coins {
        total += other_thing::value_in_cents(coin);
    }
    return total;
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn count_quarters(coins: Vec<other_thing::Coin>){
    let mut count = 0;
    for coin in coins {
        if let other_thing::Coin::Quarter(state) = coin {
            println!("Quarter is from {:?}!", state);
            count += 1;
        } else {
            println!("{:?}", coin);
        }
    }
    println!("Final count: {}", count);
}
