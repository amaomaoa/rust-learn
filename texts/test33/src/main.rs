fn foo(v: i32) {
    if v > 10 {
        return;
    }
    foo(v + 1);
    print!("{} ", v);
}
fn main() {
    foo(1);
}
