use std::{error::Error, io::stdin};

use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = TcpStream::connect("127.0.0.1:8080").await?;
    let (tx, mut rx) = mpsc::channel::<String>(3);
    tokio::spawn(async move {
        let (r, mut w) = client.split();
        let mut r = BufReader::new(r);
        let mut line = String::new();
        loop {
            match rx.recv().await {
                Some(msg) => {
                    println!("{}", msg);
                    w.write_all(msg.as_bytes()).await.unwrap();
                }
                None => {}
            }
        }
    });
    loop {
        let mut buff = String::new();
        stdin().read_line(&mut buff)?;
        tx.send(buff).await.unwrap();
    }
}
