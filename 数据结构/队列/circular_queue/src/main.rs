pub struct Queue {
    front: i32,
    rear: i32,
    queue_list: i32,
    data: Vec<i32>,
}

impl Queue {
    pub fn new(queue_list: i32) -> Self {
        Self {
            front: 0,
            rear: 0,
            queue_list: queue_list,
            data: vec![0; queue_list as usize],
        }
    }
    pub fn push(&mut self, elem: i32) {
        if (self.rear + 1) % self.queue_list == self.front {
            println!("队列已经满了");
        } else {
            self.data[(self.rear) as usize] = elem;
            self.rear = (self.rear + 1) % self.queue_list;
        }
    }
    pub fn pop(&mut self) {
        if self.front == self.rear {
            println!("队列为空");
        } else {
            self.data[self.front as usize] = 0;
            self.front = (self.front + 1) % self.queue_list;
        }
    }
}

fn print_queue(queue: &Queue) {
    for i in queue.data.iter() {
        if *i != 0 {
            print!("{}<-", i);
        }
    }
    println!("Null")
}

fn main() {
    let queue_list = 5;
    let mut queue = Queue::new(queue_list);
    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(4);
    print_queue(&queue);
    queue.pop();
    queue.pop();
    queue.pop();
    print_queue(&queue);
}
