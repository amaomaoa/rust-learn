use std::f64::consts::PI;

struct Round {
    r: i32,
}
impl Round {
    fn area(&self) -> f64 {
        (self.r as f64) * PI
    }
}
fn main() {
    println!("Hello, world!");
    let r1 = Round { r: 3 };
    print!("{}", r1.area());
}
