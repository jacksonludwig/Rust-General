use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{io, fs};

fn main() {
    let f = File::open("hello.txt");

    /*
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem open the file: {:?}", error);
        }
    };
    */

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_created) => file_created,
                Err(error_creating) => panic!("Problem creating the file: {:?}", error_creating),
            },
            other_error => panic!("Problem open the file: {:?}", other_error),
        }
    };

    // better:
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").unwrap(); // either a file returned or panic
    let f = File::open("hello.txt").expect("Failed to open."); // either a file returned or panic -- more clear panic msg

}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_better() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // returns error if there's one
    let mut s = String::new();
    f.read_to_string(&mut s)?; // returns error if there's one
    Ok(s)
}

fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; // returns error if there's one

    Ok(s)
}

fn read_username_from_file_mega_short() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}