fn power(number: i32) -> i32 {
    number * number
}
fn main() {
    for i in 0..100 {
        for j in 0..100 {
            for m in 0..100 {
                if power(i) + power(j) + power(m) == 2020 && i != 0 && j != 0 && m != 0 {
                    println!("{}+{}+{}", i, j, m);
                    return;
                }
            }
        }
    }
}
