use element::Element;
mod element;
mod data_parser;

fn main() {
    let mut elements: Vec<Element> = Vec::new();
    data_parser::build_list(&mut elements);

    for element in elements {
        println!("{}, protons: {}, electrons: {}", element.name, element.atomic_num, element.number_electrons);
    }
}




