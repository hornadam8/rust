fn main() {
    let s = String::new();
    println!("Made a new empty string: {}", s);

    let data = "initial contents";
    let s = data.to_string();
    println!("Made string from &str using to_string: {}", s);

    let s = String::from("string from &str");
    println!("Made string using from: {}", s);

    // push_str takes ownership of s2
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); // s2 dropped here
    println!("mutable string added to with push_str: {}", s);

    // push adds a single character to a string
    let mut s = String::from("lo");
    s.push('l');
    println!("added char 'l' with push: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("concatenated string: {}", s);


}
