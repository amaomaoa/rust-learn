use core::time;
use futures;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
};
use tokio::runtime::Runtime;

async fn show_ticket(name: String, ticket: &Arc<Mutex<i32>>) {
    let ticket = Arc::clone(&ticket);
    loop {
        tokio::time::delay_for(tokio::time::Duration::from_millis(10)).await;
        let mut num = ticket.lock().unwrap();
        if *num > 0 {
            *num -= 1;
            sleep(time::Duration::from_millis(100));
            println!("{}卖出1张票，还剩{}张", name, *num);
        } else {
            break;
        }
    }
}

async fn astnc_main() {
    let ticket = Arc::new(Mutex::new(100));
    let lihua = show_ticket("李华".to_string(), &ticket);
    let xiaoming = show_ticket("小明".to_string(), &ticket);
    let xiaodong = show_ticket("小东".to_string(), &ticket);
    futures::join!(lihua, xiaoming, xiaodong);
}

pub fn main() {
    let mut runtime = Runtime::new().unwrap();
    runtime.block_on(astnc_main());
}
