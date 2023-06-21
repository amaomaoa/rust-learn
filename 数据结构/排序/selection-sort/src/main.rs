fn selection_sort(v: &mut [i32]) {
    for i in 0..v.len() - 1 {
        let mut temp = i;
        for j in i..v.len() {
            if v[j] < v[temp] {
                //println!("j = {},v[j] = {}", j, v[j]);
                temp = j;
            }
        }
        v.swap(temp, i);
    }
}

fn main() {
    let mut a = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    selection_sort(&mut a);
    println!("{:?}", a);
    println!("Hello, world!");
}
