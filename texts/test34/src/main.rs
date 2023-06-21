struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Droping{}", self.name);
        if !self.nicknames.is_empty() {
            print!("{}", self.nicknames.join(","));
        }
        println!("");
    }
}

fn main() {
    let mut a = Appellation {
        name: "name".to_string(),
        nicknames: vec!["hello".to_string(), "world".to_string()],
    };
    println!("--------");
    a = Appellation {
        name: "name".to_string(),
        nicknames: vec![],
    };
}
