use std::{cell::RefCell, rc::Rc};

pub struct tree {
    head: Node,
}

pub struct child {
    root: i32,
    lchild: Node,
    rchild: Node,
}
type Node = Option<Rc<RefCell<child>>>;

impl tree {
    pub fn new(root_node: i32) -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let node = Rc::new(RefCell::new(child {
            root: elem,
            lchild: None,
            rchild: None,
        }));
        traverse_tree(self.head.clone(), Some(node));
    }
}

pub fn traverse_tree(mut_node: Node, node: Node) {
    if mut_node.clone().unwrap().borrow().lchild.is_some() {
        if mut_node.clone().unwrap().borrow().rchild.is_some() {
            let lnode = mut_node.clone().unwrap().borrow_mut().lchild.clone();
            let rnode = mut_node.clone().unwrap().borrow_mut().lchild.clone();

            traverse_tree(lnode, node.clone());
            traverse_tree(rnode, node.clone());
        } else {
            mut_node.clone().unwrap().borrow_mut().rchild = Some(node).unwrap();
            return;
        }
    } else {
        mut_node.clone().unwrap().borrow_mut().lchild = Some(node).unwrap();
        return;
    }
}

pub fn println_tree(tree: Node) {
    let data = tree.clone().unwrap().borrow().root;
    println!("{}", data);
    if tree.clone().unwrap().borrow_mut().lchild.is_some() {
        let lchild = tree.clone().unwrap().borrow_mut().lchild.clone();
        println_tree(lchild);
        if tree.clone().unwrap().borrow_mut().rchild.is_some() {
            let rchild = tree.clone().unwrap().borrow_mut().rchild.clone();
            println_tree(rchild);
        }
    } else {
        println!("Null");
        return;
    }
}

fn main() {
    let tree = tree::new(1);
    println_tree(tree.head);
}
