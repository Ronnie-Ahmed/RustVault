use std::collections::VecDeque;
struct MyStack{
    queue:VecDeque<i32>
}

impl MyStack{
    fn new()->Self{
        Self { queue: VecDeque::new() }
    }
    fn push(&mut self,x:i32){
        self.queue.push_back(x);
        let length=self.queue.len();
        for _ in 0..length-1{
            if let Some(value) = self.queue.pop_front() {
                self.queue.push_back(value);
                
            }
        }

    }
    fn pop(&mut self) -> i32 {
        
        self.queue.pop_front().unwrap()
    }

    fn top(&self)->i32{
        *self.queue.front().unwrap()
    }
    fn empty(&self)->bool{
        self.queue.is_empty()
    }
}
fn main() {
    println!("Hello, world!");
}
 // 1 2 3 add 4