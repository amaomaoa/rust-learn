use core::fmt;
#[derive(Debug)]
pub struct AverCollect {
    list: Vec<i32>,
    aver: f64,
}

impl fmt::Display for AverCollect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: String = self
            .list
            .iter()
            .map(|&x| x.to_string())
            .map(|x| -> String {
                let j = ";";
                x + j
            })
            .collect();
        write!(f, "{}", a)
    }
}
impl AverCollect {
    pub fn new() -> AverCollect {
        AverCollect {
            list: vec![],
            aver: 0.0,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.upadte_average();
    }
    pub fn getaver(&self) -> f64 {
        self.aver
    }
    pub fn remove(&mut self, value: i32) {
        let mut a: usize = 0;
        let mut if_yes: bool = false;
        for i in self.list.iter() {
            if i == &value {
                if_yes = true;
                break;
            }
            a = a + 1;
        }
        if if_yes == true {
            self.list.remove(a);
        } else {
            println!("没有这个数字");
        }
        self.upadte_average();
    }

    fn upadte_average(&mut self) {
        if self.list.len() == 0 {
            self.aver = 0.0;
        } else {
            let total: i32 = self.list.iter().sum();
            self.aver = total as f64 / self.list.len() as f64
        }
    }
}
