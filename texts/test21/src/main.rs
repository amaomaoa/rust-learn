use std::rc::Rc;
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn append(&mut self, elem: T) {
        let node = Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        });
        self.head = Some(node);
    }
}

fn main() {
    let mut list1 = List::new();
    list1.append(1);
    list1.append(1);
    println!("{:?}", list1);
}
