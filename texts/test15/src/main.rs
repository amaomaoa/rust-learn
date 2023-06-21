use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let counter = Arc::new(Mutex::new(100));
    let mut handles = vec![];
    let man = vec!["小明", "小花", "小红"];
    for i in man {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || loop {
            let mut num = counter.lock().unwrap();
            loop {
                if *num == 0 {
                    return;
                }
                *num -= 1;
                println!("{}卖出{}", i, num);
                thread::sleep(time::Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
