fn add_string(s: &mut String) -> &String {
    s.push_str(",word");
    s
}
fn main() {
    let mut s = String::from("hello");
    println!("{}", add_string(&mut s));
}
