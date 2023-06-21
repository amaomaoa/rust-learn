use std::fs::File;
use std::io::*;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating File: {:?}", e),
            },
            other => panic!("Error creating File: {:?}", other),
        },
    };
}
