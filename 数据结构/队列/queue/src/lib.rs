use std::mem::swap;

pub struct Queue {
    pub older: Vec<char>,
    pub younger: Vec<char>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
    pub fn push(&mut self, c: char) {
        self.younger.push(c)
    }
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            swap(&mut self.younger, &mut self.older);
            self.older.reverse();
        }
        self.older.pop()
    }
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.younger, self.older)
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;
    #[test]
    fn queue_test1() {
        let mut q = Queue::new();
        q.push('1');
        //assert_eq!(q.pop(), Some('1'));
        let (younger, old) = q.split();
        assert_eq!(younger, vec!['1']);
        assert_eq!(old, vec![]);
    }
    #[test]
    fn queue_test2() {
        let mut q = Queue::new();
        q.push('1');
        assert_eq!(q.pop(), Some('1'));
        assert_eq!(q.pop(), None);
    }
}
