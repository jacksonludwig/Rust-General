#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is: {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rec2 = Rectangle {width:50, height:60};
    println!("{}", rec2.can_hold(&rect1));
    println!("{}", rect1.can_hold(&rec2));
}

// creating methods for a struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    fn square(size: u32) -> Rectangle { // THIS IS A STATIC METHOD (no self)
        Rectangle {width: size, height:size}
    }
}