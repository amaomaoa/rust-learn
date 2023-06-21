use futures::{self, executor};
use std::thread::sleep;
use std::time::Duration;

async fn learn_song() {
    sleep(Duration::from_secs(1));
    println!("learn_song");
}

async fn sing_song() {
    println!("sing_song");
}

async fn dance() {
    println!("dance");
}

async fn learn_song_and_sing_song() {
    learn_song().await;
    sing_song().await;
}

async fn async_main() {
    let f1 = dance();
    let f2 = learn_song_and_sing_song();
    futures::join!(f2, f1);
}

fn main() {
    executor::block_on(async_main());
}
