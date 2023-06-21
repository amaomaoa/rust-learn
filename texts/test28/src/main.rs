use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
use crate::List::{Cons, Nil};
pub fn main() {
    let a = Rc::new(Cons(10, RefCell::new(Rc::new(Nil))));
    {
        let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));

        println!("{}", Rc::strong_count(&b));
        if let Some(List) = a.tail() {
            *List.borrow_mut() = Rc::clone(&b);
        }
    }
    println!("{}", Rc::strong_count(&a));
    //println!("{:?}", a.tail());
}
