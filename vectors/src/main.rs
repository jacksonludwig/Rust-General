fn main() {
    let mut vect: Vec<i32> = Vec::new();
    vect.push(6);
    vect.push(124);
    println!("{:?}", vect);

    println!();

    let mut vect_inferred = vec![1, 2, 3];
    let third_element = &vect_inferred[2]; // gives reference -- you want it to crash if out of bounds
    let a = vect_inferred[2]; // copies value into a

    match vect_inferred.get(2) { // get gives Option wrapped around reference -- you dont want it to crash if out of bounds
        Some(third_element) => println!("The third element is {}", third_element),
        None => println!("There is no element there"),
    }

    for num in &mut vect_inferred {
        *num += 50;
    }

    for num in &mut vect_inferred {
        println!("{}", num);
    }

    // -------------------------------
    let row = vec![ // using an enum wrapper to store dif types of values in a vector
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}