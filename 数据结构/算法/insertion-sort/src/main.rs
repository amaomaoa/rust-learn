use std::time::Instant;

use rand::Rng;

fn insertion_sort(list: &mut Vec<i32>) {
    for i in 1..list.len() {
        let mut pre_index = i - 1;
        let current = list[i];
        while pre_index as i32 >= 0 && list[pre_index] > current {
            list[pre_index + 1] = list[pre_index];
            pre_index = pre_index - 1;
        }
        list[pre_index + 1] = current;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut list = vec![];
    for _i in 0..100000 {
        let x = rng.gen_range(0..100);
        list.push(x);
    }
    //println!("{:?}", list);
    let start = Instant::now();
    insertion_sort(&mut list);
    let end = Instant::now();
    let duration = end - start;
    //println!("{:?}", list);
    println!("{:?}", duration);
    println!("Hello, world!");
}
