fn quick_sort(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    //let p = thread_rng().gen_range(0..nums.len() - 1);
    //nums.swap(p, left);
    let mut l = left;
    let mut r = right;
    while l < r {
        while l < r && nums[r] > nums[left] {
            r -= 1;
        }
        while l < r && nums[l] < nums[left] {
            l += 1;
        }
        nums.swap(l, r);
    }
    nums.swap(left, l);
    if l > 1 {
        quick_sort(nums, left, l - 1);
    }

    quick_sort(nums, r + 1, right);
}

pub fn main() {
    let mut a = vec![3, 2, 1];
    let len = a.len().clone();
    //let start = Instant::now();
    quick_sort(&mut a, 0, len);
    println!("{:?}", a);
}
