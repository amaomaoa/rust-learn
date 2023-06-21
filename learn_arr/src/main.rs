fn arr_add<F: FnMut(i32, &i32) -> i32>(v: &[i32], a: i32, mut f: F) -> i32 {
    let mut accum = a;
    for i in v.iter() {
        accum = accum + *i;
    }
    accum
}

fn main() {
    let mut arr = [213, 321, 21, 3, 31, 2, 31412, 3, 23];
    arr.sort();
    let v = vec![2, 3, 5, 7];
    let f = arr_add(&v, 0, |a, b| a + b);
    println!("arr_add = {}", f);
    let c = v.iter().fold(0, |a, b| a + b);
    println!("{}", c);
}
