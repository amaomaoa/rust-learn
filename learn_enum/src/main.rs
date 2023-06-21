use std::cmp::Ordering::*;

fn foo(x: i32, y: i32) -> &'static str {
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "at the x axis",
        (Equal, _) => "at the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        (Less, Less) => "in the third quadrant",
        (Greater, Less) => "in the fourth quadrant",
    }
}

fn main() {
    println!("{}", foo(0, 0));
    println!("{}", foo(-1, 5));
    println!("{}", foo(8, -9));
}
