
// value_in_cents takes a coin and returns its value in cents
// ```rust
// let val = value_in_cents(Coin::Penny);
// assert!(val == 1);
// ```
pub fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            2
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}", state);
            25
        },
    }
}

#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
pub enum UsState{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia, //10
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland, //20
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey, //30
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota, // 40
    Tennessee,
    Texas,
    Utah,
    Virginia,
    Vermont,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming // 50!
}
