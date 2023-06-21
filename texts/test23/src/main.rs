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
    pub fn push_tail(&mut self, elem: i32) {
        let node = Rc::new(RefCell::new(Node {
            data: elem,
            next: None,
        }));
        if self.head.is_none() {
            self.head = Some(node);
            return;
        }
        let mut next = self.head.clone().unwrap();
        loop {
            if next.borrow_mut().next.is_none() {
                next.borrow_mut().next = Some(node);
                return;
            } else {
                next = next.clone().borrow_mut().next.clone().unwrap();
            }
        }
    }
    pub fn push_head(&mut self, elem: i32) {
        let node = Rc::new(RefCell::new(Node {
            data: elem,
            next: self.head.clone(),
        }));
        self.head = Some(node);
    }
    pub fn pop(&mut self, elem: i32) {
        if self.head.is_none() {
            println!("链表为空");
            return;
        }

        if self.head.clone().unwrap().borrow_mut().data == elem {
            self.head = self.head.clone().unwrap().borrow_mut().next.clone();
            return;
        }
        let mut next = self.head.clone().unwrap();
        loop {
            if next.borrow_mut().next.clone().unwrap().borrow().data == elem {
                let next2 = next
                    .clone()
                    .borrow()
                    .next
                    .clone()
                    .unwrap()
                    .borrow()
                    .next
                    .clone();
                next.borrow_mut().next = next2;
                return;
            } else {
                next = next.clone().borrow_mut().next.clone().unwrap();
            }
        }
    }
}

pub fn print_list(list: &List) {
    if list.head.is_none() {
        println!("Null");
        return;
    }
    let mut next = list.head.clone().unwrap();
    loop {
        if next.borrow_mut().next.is_none() {
            println!("{}->Null", next.borrow_mut().data);
            return;
        } else {
            print!("{}->", next.borrow_mut().data);
            next = next.clone().borrow_mut().next.clone().unwrap();
        }
    }
}

fn main() {
    let mut list1 = List::new();
    list1.push_tail(1);
    list1.push_tail(2);
    list1.push_tail(3);
    list1.pop(2);
    print_list(&list1);
}
