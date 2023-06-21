use rand::{thread_rng, Rng};
use std::time::Instant;

fn quick_sort3(v: &mut [i32]) {
    if v.len() < 2 {
        return;
    }
    let p = thread_rng().gen_range(0..v.len() - 1);
    let (mut l, mut r) = (0, v.len() - 1);
    v.swap(l, p);
    let temp = v[l];
    while l < r {
        while l < r {
            if temp > v[r] {
                v[l] = v[r];
                break;
            }
            r -= 1;
        }
        while l < r {
            if temp < v[l] {
                v[r] = v[l];
                break;
            }
            l += 1;
        }
    }
    v[l] = temp;
    let (a, b) = v.split_at_mut(l + 1);
    rayon::join(|| quick_sort3(a), || quick_sort3(b));
}

fn main() {
    let mut a = vec![];
    for _i in 0..1000000 {
        a.push(thread_rng().gen_range(0..10000));
    }
    let start = Instant::now();
    quick_sort3(&mut a);
    let end = Instant::now();
    let duration = end - start;
    println!("{:?}", duration);
    println!("Hello, world!");
}
