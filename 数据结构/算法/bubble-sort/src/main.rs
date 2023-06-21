use rand::Rng;
use std::time::Instant;

fn bubble_sort(list: &mut Vec<i32>) {
    for i in 1..list.len() {
        for j in 0..list.len() - i {
            if list[j] > list[j + 1] {
                let temp = list[j + 1];
                list[j + 1] = list[j];
                list[j] = temp;
            }
        }
    }
}
fn main() {
    let mut rng = rand::thread_rng();
    let mut list = vec![];

    for _i in 0..100000 {
        let x = rng.gen_range(0..100);
        list.push(x);
    }
    let start = Instant::now();
    bubble_sort(&mut list);
    let end = Instant::now();
    let duration = end - start;
    println!("{:?}", duration);
    println!("Hello, world!");
}
