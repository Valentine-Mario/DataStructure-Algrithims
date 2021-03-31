use std::fmt;

#[derive(Debug)]
pub struct Queue<T> {
    pub size: usize,
    pub item: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(maxsize: usize) -> Self {
        Self {
            size: maxsize,
            item: Vec::with_capacity(maxsize),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        if self.size == self.get_size() {
            println!("queue is full")
        } else {
            self.item.push(value)
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.item.remove(0))
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.item.len() == 0
    }

    pub fn get_size(&mut self) -> usize {
        self.item.len()
    }
}

#[derive(Debug)]
pub struct MyCircularQueue<T> {
    pub maxsize: usize,
    pub item: Vec<T>,
    pub head: i32,
    pub tail: i32,
}

impl<T> MyCircularQueue<T>
where
    T: fmt::Display,
{
    pub fn new(maxsize: usize) -> Self {
        Self {
            maxsize,
            item: Vec::with_capacity(maxsize),
            head: -1,
            tail: -1,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        //check if queue is empty
        if (self.tail + 1 % self.maxsize as i32) == self.head {
            println!("The circular queue is full");
        } else if self.head == -1 {
            self.head = 0;
            self.tail = 0;
            self.item[self.tail as usize] = value
        } else {
            self.tail = (self.tail + 1) % self.maxsize as i32;
            self.item[self.tail as usize] = value
        }
    }

    pub fn dequeue(&mut self) -> Option<&T> {
        if self.head == -1 {
            println!("queue is empty");
            None
        } else if self.head == self.tail {
            let temp = &self.item[self.head as usize];
            self.head = -1;
            self.tail = -1;
            return Some(temp);
        } else {
            let temp = &self.item[self.head as usize];
            self.head = (self.head + 1) % self.maxsize as i32;
            return Some(temp);
        }
    }
    pub fn printCircularQueue(&self) {
        if self.head == -1 {
            println!("circular queue is empty")
        } else {
            for elem in self.item.iter() {
                println!("Item: {}", elem)
            }
        }
    }
}
