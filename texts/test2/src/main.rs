#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
}

trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> i32;
}

//<T: GetName + GetAge>里面是约束
fn print_information<T: GetName + GetAge>(item: T) {
    println!("name = {},age = {}", item.get_name(), item.get_age());
}

fn creating_student() -> Student {
    Student {
        name: String::from("xiaoming"),
        age: 15,
    }
}

impl GetAge for Student {
    fn get_age(&self) -> i32 {
        self.age
    }
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}
pub fn main() {
    let s = Student {
        name: "lihua".to_string(),
        age: 18,
    };
    print_information(s);
    println!("{:?}", creating_student());
}
