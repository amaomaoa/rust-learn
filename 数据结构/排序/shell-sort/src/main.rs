fn shell_sort(arr: &mut [i32]) {
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
            println!("{:?}", arr);
        }
        gap /= 2;
    }
}

fn main() {
    let mut a = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    shell_sort(&mut a);
    println!("{:?}", a);
}
