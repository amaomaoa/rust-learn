use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}
type Link<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}
impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}
impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    pub fn push_front(&mut self, elem: T) {
        let node = Node::new(elem);
        match self.head.take() {
            Some(head) => {
                node.borrow_mut().next = Some(head.clone());
                head.borrow_mut().prev = Some(node.clone());
                self.head = Some(node)
            }
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
        }
    }
}
fn main() {
    let mut list1 = List::new();
    list1.push_front(1);
    //list1.push_front(2);
    println!("{:?}", list1)
}
