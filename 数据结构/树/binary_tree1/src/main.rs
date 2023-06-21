use ::std::process;
#[derive(Debug, Default)]
struct Tree {
    value: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    fn get_val(&self) -> i32 {
        return self.value;
    }
    fn set_val(&mut self, val: i32) -> i32 {
        self.value = val;
        return self.value;
    }
    fn insert(&mut self, dir: &String, val: Tree) {
        assert!(dir == "left" || dir == "right");
        match dir.as_ref() {
            "left" => self.left = Some(Box::new(val)),
            "right" => self.right = Some(Box::new(val)),
            _ => {
                println!("Insert Error: only left and right supported");
                process::exit(1);
            }
        }
    }
    fn delete(&mut self, dir: &String) {
        assert!(dir == "left" || dir == "right");
        match dir.as_ref() {
            "left" => self.left = None,
            "right" => self.right = None,
            _ => {
                println!("Insert Error: only left and right supported");
                process::exit(1);
            }
        }
    }
}

// 非消耗性遍历
fn traverse(tree: &Tree) {
    println!("Node Value: {:?}", tree.value);
    match tree.left {
        Some(ref x) => traverse(x),
        _ => {}
    }
    match tree.right {
        Some(ref x) => traverse(x),
        _ => {}
    }
}

fn main() {
    println!("begin rust tree test:");
    let mut tree = Tree {
        value: 12,
        ..Default::default()
    };
    let mut left = Tree {
        value: 121,
        ..Default::default()
    };
    tree.insert(&String::from("left"), left);
    let mut right = Tree {
        value: 122,
        ..Default::default()
    };
    tree.insert(&String::from("right"), right);
    traverse(&tree);
}
