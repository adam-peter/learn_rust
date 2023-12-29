fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{v:?}");

    let v = vec![1, 2, 3];
    println!("{v:?}");

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{v:?}");

    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2]; // indexed element should be a reference
    println!("Third element: {third}");
    let third = v.get(2); // returns an optional reference
    match third {
        Some(num) => println!("Third element: {num}"),
        None => println!("There is no third element.")
    }

    let mut v = vec![100, 32, 57];
    println!("{v:?}");
    for n in &v {
        let n_plus_one = n + 1;
        println!("{n_plus_one}");
    }
    for n in &mut v {
        *n += 50;
    }
    println!("{v:?}");

    // safely iterating over vectors using iterators
    let mut v = vec![1, 2];
    let mut iter = v.iter(); // pointer to first element
    let n1 = iter.next().unwrap(); // next moves the pointer to next element, returns optional reference to previous element
    let n2 = iter.next().unwrap();
    let end = iter.next();
    println!("v: {v:?}\n\
    n1: {n1}\n\
    n2: {n2}\n\
    end: {end:?}");

    // using an enum to store values of different types in a vec
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    println!("{row:?}");


}

fn dup_in_place(v: &mut Vec<i32>) {
    // for n_ref in v.iter() { // iterators give you an immutable reference - don't let you mutate vectors
    //     v.push(*n_ref);
    // }
}
