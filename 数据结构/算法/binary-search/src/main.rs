use std::time::Instant;

fn binary_search(list: &Vec<i32>, target: i32) -> usize {
    let mut low = 0;
    let mut high = list.len();
    let mut count = 0;

    loop {
        count = count + 1;
        if low < high {
            let mid = (low + high) / 2;
            if list[mid] == target {
                println!("一共查找了{}次", count);
                return mid;
            } else if list[mid] > target {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
    }
}

fn main() {
    let list: Vec<i32> = (1..2147483647).collect();
    let a = [1, 2, 3];

    let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();
    {
        let start = Instant::now();
        println!("mid = {}", binary_search(&list, 30));
        let end = Instant::now();
        let duration = end - start;
        println!("{:?}", duration);
        println!("----------------------");
    }
    {
        let start = Instant::now();
        println!("{}", list.binary_search_by(|x| x.cmp(&30)).unwrap());
        let end = Instant::now();
        let duration = end - start;
        println!("{:?}", duration);
    }
}
