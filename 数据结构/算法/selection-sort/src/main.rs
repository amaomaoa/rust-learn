use std::time::Instant;

use rand::Rng;

fn selection_sort(list: &mut Vec<i32>) {
    for i in 0..list.len() {
        let mut min_idex = i;
        for j in (i + 1)..list.len() {
            if list[j] < list[min_idex] {
                min_idex = j;
            }
        }
        if i != min_idex {
            let temp = list[i];
            list[i] = list[min_idex];
            list[min_idex] = temp;
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
    selection_sort(&mut list);
    let end = Instant::now();
    let duration = end - start;
    println!("{:?}", duration);
    println!("Hello, world!");
}
