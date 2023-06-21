use rand::{thread_rng, Rng};

fn pivot<T: Ord>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}
fn quick_sort<T: std::fmt::Debug + Ord>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}
pub fn main() {
    let mut a = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    quick_sort(&mut a);
    println!("{:?}", a);
}
