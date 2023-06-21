fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn put_hello(a: &mut String) {
    a.push_str("hello");
}

fn main() {
    let a = "long";
    //let b = "sdasdas";
    let mut c = String::from("hello");
    put_hello(&mut c);
    println!("{}", c);
    println!("{}", longer(a, &c))
}
