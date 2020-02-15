fn main() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("not three"),
    }

    // better way:
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}
