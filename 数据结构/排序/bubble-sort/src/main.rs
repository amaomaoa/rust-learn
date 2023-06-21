fn bubble_sort(v: &mut [i32]) {
    for i in 1..v.len() {
        let mut p = i - 1;
        let temp = v[i];
        while p as i32 >= 0 && temp < v[p] {
            v[p + 1] = v[p];
            p -= 1;
        }
        v[p + 1] = temp;
    }
}

fn main() {
    let mut a = [9, 3, 2, 4, 2, 523, 52, 3, 42];
    bubble_sort(&mut a);
    println!("{:?}", a);
}
