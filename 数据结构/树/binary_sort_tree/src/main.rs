use std::{cell::RefCell, rc::Rc};

struct Tree {
    root: Link,
}

struct Node {
    data: i32,
    lchild: Link,
    rchild: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Tree {
    pub fn new(root: i32) -> Self {
        let link = Rc::new(RefCell::new(Node {
            data: root,
            lchild: None,
            rchild: None,
        }));
        Self { root: Some(link) }
    }

    pub fn push(&mut self, data: i32) {}
}

fn main() {
    println!("Hello, world!");
}
