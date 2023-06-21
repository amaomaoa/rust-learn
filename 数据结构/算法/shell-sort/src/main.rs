//use std::time::Instant;

use std::time::Instant;

use rand::Rng;

fn shell_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    let mut gap = len as i32 / 2;

    while gap > 0 {
        for i in gap..len as i32 {
            let temp = arr[i as usize];
            let mut j = i;

            while j >= gap && arr[j as usize - gap as usize] > temp {
                arr.swap(j as usize, j as usize - gap as usize);
                j -= gap;
            }

            arr[j as usize] = temp;
        }
        gap /= 2;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut list = vec![];
    for _i in 0..10000000 {
        let x = rng.gen_range(0..100);
        list.push(x);
    }
    //println!("{:?}", list);
    let start = Instant::now();
    shell_sort(&mut list);
    //println!("{:?}", list);
    let end = Instant::now();
    let duration = end - start;
    println!("{:?}", duration);
    println!("Hello, world!");
}
