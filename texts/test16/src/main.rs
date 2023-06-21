use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time;

fn main() {
    let ticket = Arc::new(Mutex::new(100));
    let man = vec!["小明", "小花", "小东"];
    let mut headles = vec![];

    for a in man {
        let ticket = Arc::clone(&ticket);
        let headle = thread::spawn(move || loop {
            sleep(time::Duration::from_millis(1));
            let mut num = ticket.lock().unwrap();
            if *num > 0 {
                *num -= 1;
                thread::sleep(time::Duration::from_millis(100));
                println!("{}卖出1张票，剩余{}张", a, *num);
            } else {
                break;
            }
        });
        headles.push(headle);
    }
    for headle in headles {
        headle.join().unwrap();
    }
}
