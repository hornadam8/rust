fn main() {
    let v: Vec<i32> = Vec::new();
    println!("A new empty vec: {v:?}");

    let v = vec![1, 2, 3];
    println!("A new vec made with vec!: {v:?}");

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    v.pop();
    println!("A mutable vec populated with push, last element removed with pop: {v:?}");

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("ref to the third element of a vector {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(element) => println!("Safely got third element with get: {element}"),
        None => println!("Safely searched a null index")
    }

    let sixth: Option<&i32> = v.get(5);
    match sixth {
        Some(element) => println!("Safely got third element with get: {element}"),
        None => println!("Safely tried accessing an out of bounds index")
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("element in vector: {i}")
    }

    let mut v = vec![100, 37, 52];
    for i in &mut v {
        println!("adding 50 to element in mutable vector {i}");
        *i += 50;
    }
    println!("mut vec after loop: {v:?}");

    // enum spreadsheet cell defined on line 93
    let v = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Hello!"))
    ];

    for i in &v {
        match i {
            SpreadsheetCell::Int(ele) => println!("{ele} is an int!"),
            SpreadsheetCell::Float(ele) => println!("{ele} is a float!"),
            SpreadsheetCell::Text(ele) => println!("{ele} is text!")
        }
    }

    // dropping a vector also drops its elements
    {
        let mut v = vec![1, 2, 3, 4];
        let last = v.last();
        match last {
            Some(i) => println!("got last element: {i}"),
            None => println!("failed to get last element")
        }
        // removing all but one element
        v.pop();
        v.pop();
        v.pop();

        // removing last element
        let last = v.pop();
        match last {
            Some(i) => println!("got last element: {i}"),
            None => println!("failed to get last element")
        }


        let last = v.last();
        match last {
            Some(i) => println!("got last element: {i}"),
            None => println!("failed to get last element, vec empty")
        }

        // adding an element that will go out of scope immediately due to the end of the closure
        v.push(42);
    } // <- v and it's contents goes out of scope and is freed here

} // </main>

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}