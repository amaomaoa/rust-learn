#[derive(Debug)]
struct Dog {
    name: String,
    weight: f64,
}

impl Dog {
    fn set_weight(&mut self, weight: f64) {
        self.weight = weight
    }
    fn set_name(&mut self, name: String) {
        self.name = name
    }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_weight(&self) -> f64 {
        self.weight
    }
    fn show() {
        println!("oh oh oh")
    }
}

pub fn main() {
    let mut dog = Dog {
        name: String::from("xiaohuo"),
        weight: 88.8,
    };
    println!("{:#?}", dog);
    dog.set_weight(90.0);
    dog.set_name(String::from("xiaoming"));
    println!("{:#?}", dog);
    println!("{},{}", dog.get_name(), dog.get_weight());
    Dog::show()
}
