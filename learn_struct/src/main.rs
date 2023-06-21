fn main() {
    let square1 = Rectangle::square(20);
    let rect1 = Rectangle {
        width: 10,
        length: 11,
    };
    let rect2 = Rectangle {
        width: 12,
        length: 13,
    };
    let rect3 = Rectangle {
        width: 1,
        length: 4,
    };
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
    println!("{}", square1.area());

    println!(
        "{}",
        Rectangle {
            width: 10,
            length: 20,
        }
        .area()
    )
}

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}
