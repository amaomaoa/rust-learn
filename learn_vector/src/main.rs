fn main() {
    let v = vec![1, 2, 3];
    for i in 0..5 {
        match v.get(i) {
            Some(_) => println!("{}", v[i]),
            None => println!("None"),
        }
    }
}
