#[derive(Debug)]
struct Point<T> {
    x: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let integer = Point { x: 5 };
    println!("{:?}", integer);
}
