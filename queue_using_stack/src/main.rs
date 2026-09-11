use std::vec;

#[derive(Debug)]
pub struct MyQueue {
    elements: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.elements.push(x);
    }
    fn pop(&mut self) -> i32 {
        let length = self.elements.len();
        let mut temp: Vec<i32> = Vec::new();
        for _ in 0..length - 1 {
            if let Some(value) = self.elements.pop() {
                temp.push(value);
            }
        }
        let return_value = self.elements.pop().unwrap();

        for num in temp.iter().rev() {
            self.elements.push(*num);
        }
        return_value
    }

    fn peek(&mut self) -> i32 {
        let mut temp: Vec<i32> = Vec::new();
        let length = self.elements.len();
        for _ in 0..length {
            if let Some(value) = self.elements.pop() {
                temp.push(value);
            }
        }
        let return_value = temp[temp.len()-1];
        for num in temp.iter().rev() {
            self.elements.push(*num);
        }
        return_value
    }

    fn empty(&self) -> bool {
        self.elements.is_empty()
    }
}

fn main() {
    let mut test = MyQueue::new();
    test.push(1);
    test.push(2);
    test.push(3);
    println!("{:?}", test);
    test.pop();
    println!("{:?}", test);
    println!("{}", test.peek());
    println!("{}", test.empty());
}

// Stack LIFO , Queue FIFO
