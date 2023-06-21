trait IsEmoji {
    fn is_enoji(&self) -> bool;
}

impl IsEmoji for String {
    fn is_enoji(&self) -> bool {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
