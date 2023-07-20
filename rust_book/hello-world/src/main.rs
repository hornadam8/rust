fn main() {
    println!("Hello, world!");

    {
        let x: i32 = 0;
        // x is in scope here and can be used
    } // x is dropped from memory here
    let x: i32 = 0;

    let y = if x == 0 {
        x += 1
    } else {
        x -= 1
    };
}
