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
        self.item.push(value)
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
        self.size
    }
}
