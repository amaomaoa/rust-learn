use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
pub struct List {
    head: Link,
}
type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node {
    data: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn push(&mut self, elem: i32) {
        let node = Rc::new(RefCell::new(Node {
            data: elem,
            next: None,
        }));
        let heads = self.head.take();
        if heads.is_none() {
            self.head = Some(node);
            return;
        }
        let mut heads = heads.unwrap();
        self.head = Some(heads.clone());
        loop {
            if heads.borrow_mut().next.is_none() {
                heads.borrow_mut().next = Some(node);
                return;
            } else {
                let t = heads.borrow_mut().next.clone();
                heads = t.unwrap();
            }
        }
    }
}

fn main() {
    let mut list1 = List::new();
    list1.push(1);
    list1.push(2);
    list1.push(3);
    println!("{:?}", list1);
}
