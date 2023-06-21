use std::rc::Rc;

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
struct Node {
    head: Link,
    elem: i32,
}

type Link = Option<Rc<Node>>;

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn push(&mut self, number: i32) {
        let node = Rc::new(Node {
            head: self.head.take(),
            elem: number,
        });
        self.head = Some(node);
    }
    pub fn append(&mut self, list: List) {}
}

fn main() {
    let mut list1 = List::new();
    let mut list2 = List::new();
    println!("{:?}", list1);
    list1.push(1);
    list1.push(2);
    list1.push(3);
    list2.push(4);
    list2.push(5);
    list2.push(6);
    println!("{:?}", list1);
    list1.append(list2);
    println!("{:?}", list1);
}
