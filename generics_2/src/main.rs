fn main() {
    let int = Point {x: 5, y: 10};
    let float = Point {x: 5.5, y: 10.5};

    let int = PointTwoType {x: 5, y: 10};
    let both = PointTwoType {x: 5.5, y: 10};

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}\n", p.get_x());

    let p1 = PointTwoType { x: 5, y: 10.4 };
    let p2 = PointTwoType { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

struct Point<T> {
    x: T,
    y: T,
}

struct PointTwoType<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> PointTwoType<T, U> {
    fn mixup<V, W>(self, other: PointTwoType<V, W>) -> PointTwoType<T, W> {
        PointTwoType {
            x: self.x,
            y: other.y,
        }
    }
}