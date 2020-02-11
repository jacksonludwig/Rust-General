use std::{fs, process};
use crate::element::Element;
use std::error::Error;

fn read_element_file() -> String {
    let raw_data = fs::read_to_string("D:\\Rust-General\\periodic_table\\src\\elements.csv")
        .expect("Error reading file");
    raw_data
}

fn turn_to_struct(list: &mut Vec<Element>) -> Result<(), Box<dyn Error>> {
    let raw_data = read_element_file();
    let mut reader = csv::Reader::from_reader(raw_data.as_bytes());
    for result in reader.deserialize() {
        let element: Element = result?;
        list.push(element);
    }
    Ok(())
}

pub(crate) fn build_list(list: &mut Vec<Element>) -> () {
    if let Err(err) = turn_to_struct(list) {
        println!("{}", err);
        process::exit(1);
    }
}

