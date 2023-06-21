fn foo(n: i32) {
    if n <= 10 {
        foo(n + 1);
    } else {
        return;
    }
    println!("{}", n);
}

fn main() {
    foo(1);
}
