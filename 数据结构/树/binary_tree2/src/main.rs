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
        let node = Rc::new(RefCell::new(Node {
            data: root,
            lchild: None,
            rchild: None,
        }));
        Self { root: Some(node) }
    }
    pub fn push(&mut self, elem: i32) {
        let node = Rc::new(RefCell::new(Node {
            data: elem,
            lchild: None,
            rchild: None,
        }));
        let link = self.root.clone();
        push_tree(link, Some(node));
    }
    pub fn print(&self) {
        let link = self.root.clone();
        print_tree(link);
    }
}
fn print_tree(tree: Link) {
    print!("{}  ", tree.clone().unwrap().clone().borrow().data);
    if tree.clone().unwrap().borrow().lchild.is_some() {
        let ltree = tree.clone().unwrap().borrow().lchild.clone();
        print_tree(ltree);
    }
    if tree.clone().unwrap().borrow().rchild.is_some() {
        let rtree = tree.clone().unwrap().borrow().rchild.clone();
        print_tree(rtree);
    }
}

fn push_tree(tree: Link, node: Link) {
    if tree.clone().unwrap().borrow().lchild.is_some() {
        if tree.clone().unwrap().borrow().rchild.is_some() {
            let ltree = tree.clone().unwrap().borrow().rchild.clone();
            push_tree(ltree, node);
        } else {
            tree.unwrap().clone().borrow_mut().rchild = node;
            return;
        }
    } else {
        tree.unwrap().clone().borrow_mut().lchild = node;
        return;
    }
}

fn main() {
    let mut tree = Tree::new(1);
    tree.push(2);
    tree.push(3);
    tree.push(4);
    tree.push(5);
    tree.print();
}
