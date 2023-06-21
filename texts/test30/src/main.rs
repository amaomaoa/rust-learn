fn foo<T>(v: &[i32], temp: i32, f: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    let mut a = temp;
    for i in v.iter() {
        a = f(*i, a)
    }
    a
}

fn main() {
    let a = [5, 4, 3, 2, 1];
    println!("{}", foo(&a, 0, |x, y| x + y));
}
