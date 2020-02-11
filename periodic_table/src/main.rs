use element::Element;
use parse_data::build_list;
mod element;
mod parse_data;

fn main() {
    let mut elements: Vec<Element> = Vec::new();
    build_list(&mut elements);

    for element in elements {
        println!("{}, protons: {}, electrons: {}", element.name, element.atomic_num, element.number_electrons);
    }
}




