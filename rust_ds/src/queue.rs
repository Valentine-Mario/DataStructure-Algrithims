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
pub struct CircularQueue<T> {
    pub maxsize: usize,
    pub item: Vec<T>,
    pub head: i32,
    pub tail: i32,
    pub count: i32,
}

impl<T> CircularQueue<T>
where
    T: std::fmt::Display,
{
    pub fn new(maxsize: usize) -> Self {
        Self {
            maxsize,
            item: Vec::with_capacity(maxsize),
            head: -1,
            tail: -1,
            count: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        //check if queue is empty
        if self.count == self.maxsize as i32 {
            println!("The circular queue is full");
        } else if self.count == 0 {
            self.head = 0;
            self.tail = 0;
            self.item.insert(self.tail as usize, value);
            self.tail = (self.tail + 1) % self.maxsize as i32;
            self.count += 1;
        } else {
            self.item.insert(self.tail as usize, value);
            self.tail = (self.tail + 1) % self.maxsize as i32;
            self.count += 1;
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.count == 0 {
            println!("queue is empty");
            None
        } else {
            let temp = self.item.remove(self.head as usize);
            self.head = (self.head + 1) % self.maxsize as i32;
            self.count -= 1;
            Some(temp)
        }
    }
    pub fn print_circular_queue(&self) {
        if self.count == 0 {
            println!("circular queue is empty")
        } else {
            for elem in self.item.iter() {
                println!("Item: {}", elem)
            }
        }
    }
}
