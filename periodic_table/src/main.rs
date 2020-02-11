#![windows_subsystem = "windows"]
use std::{fs, process};
use std::collections::HashMap;
use std::error::Error;
use std::io;

use element::Element;

mod element;
fn main() {
    let mut elements: Vec<Element> = Vec::new();
    build_list(&mut elements);

    let element_map = build_map(&mut elements);

    show_prompts(&element_map);
}

fn read_element_file() -> String {
    let raw_data = fs::read_to_string("resources/elements.csv")
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

fn build_list(list: &mut Vec<Element>) -> () {
    if let Err(err) = turn_to_struct(list) {
        println!("{}", err);
        process::exit(1);
    }
}

fn build_map(list: &mut Vec<Element>) -> HashMap<String, &Element> {
    let mut map: HashMap<String, &Element> = HashMap::new();
    for element in list {
        map.insert(element.name.to_string().to_ascii_lowercase(), element);
        map.insert(element.symbol.to_string().to_ascii_lowercase(), element);
        map.insert(element.atomic_num.to_string(), element);
    }
    map
}

fn show_prompts(elements: &HashMap<String, &Element>) {
    let quit_tag_1: String = String::from("quit");
    let quit_tag_2: char = 'q';

    loop {
        println!("\t\t------------Periodic Table------------");
        println!("Enter an element's symbol, name, or atomic number to receive it's information.\n(q)uit to exit.");

        let mut search = String::new();
        io::stdin().read_line(&mut search).expect("Error reading input");
        let search: String = match search.trim().to_ascii_lowercase().parse() {
            Ok(search_input) => search_input,
            Err(_) => continue,
        };

        match search {
            search if search.eq(&quit_tag_1) || search.eq(&quit_tag_2.to_string()) => process::exit(0),
            search if search.len() < 1 => continue,
            _ => ()
        }

        if elements.get(&search) != None {
            let element: &Element = elements.get(&search).unwrap();
            show_info(element);
        }
    }
}

fn show_info(element: &Element) {
    println!("Name: {}", element.name);
    println!("Symbol: {}", element.symbol);
    println!("Atomic Number: {}", element.atomic_num);
    println!("Molar mass: {}g/mol", element.mass_per_mole);
    println!("Number of Protons: {}", element.number_protons);
    println!("Number of Neutrons: {}", element.number_neutrons);
    println!("Number of Electrons: {}", element.number_electrons);
    println!("Category: {}", element.elem_type);
    if element.group != None {
        println!("Column: {}", element.group.unwrap());
    }
    if element.number_isotopes != None {
        println!("Number of Isotopes: {}", element.number_isotopes.unwrap());
    }
    if element.number_valence != None {
        println!("Number of Valence: {}", element.number_valence.unwrap());
    }
}





