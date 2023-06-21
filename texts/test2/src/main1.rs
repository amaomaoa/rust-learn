pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> i32;
}
pub struct Student {
    pub name: String,
    pub age: i32,
}
pub struct Teacher {
    pub name: String,
    pub age: i32,
    pub subject: String,
}
impl GetInformation for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> i32 {
        self.age
    }
}

trait SchoolName {
    fn get_school_name(&self) -> String {
        "hongxingxiaoxue".to_string()
    }
}
//默认实现
impl SchoolName for Student {}
impl SchoolName for Teacher {
    fn get_school_name(&self) -> String {
        "guanmingxiaoxue".to_string()
    }
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> i32 {
        self.age
    }
}
fn print_information(items: impl GetInformation) {
    println!("name = {},age = {}", items.get_name(), items.get_age());
}
fn main() {
    let t = Teacher {
        name: "lihua".to_string(),
        age: 18,
        subject: "语文".to_string(),
    };
    let s = Student {
        name: "xiaoming".to_string(),
        age: 14,
    };
    println!("schoolname = {}", t.get_school_name());
    print_information(t);
    println!("schoolname = {}", s.get_school_name());
    print_information(s);
}
