use std::{fs::File, io::Write};

fn main() {
    let mut file = match File::open("./hello.txt") {
        Ok(file) => file,
        Err(_) => match File::create("./hello.txt") {
            Ok(file) => file,
            Err(err) => panic!("{}", err),
        },
    };
    file.write_all("hello,world!!".as_bytes()).unwrap();
}
