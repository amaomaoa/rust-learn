fn function<'a, T: std::fmt::Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("{}", ann);
    if x.len() < y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let s1 = String::from("i am s1");
    let s2 = String::from("i am s2");
    let ann = 129;
    let r = function(&s1, &s2, ann);
    println!("r= {}", r);
    println!("Hello, world!");
}
