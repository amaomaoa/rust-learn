#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

#[derive(Debug)]
pub struct IntoInter<T>(List<T>);
impl<T> List<T> {
    pub fn into_iter(self) -> IntoInter<T> {
        IntoInter(self)
    }
}

impl<T> Iterator for IntoInter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

fn main() {
    let mut list1 = List::new();
    list1.push("hello");
    list1.push("word");
    println!("{:?}", list1);
}
