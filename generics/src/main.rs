use std::fmt::Display;

fn main() {
    println!("{}", largest(&vec![5, 2, 1, 5, 200, 15]));
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { // only those that implement PartialOrd and Copy may use this
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// ---------------------
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}