#[derive(Debug)]
pub struct Stack<T> {
    pub size: usize,
    pub stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(maxsize: usize) -> Self {
        Self {
            size: maxsize,
            stack: Vec::with_capacity(maxsize),
        }
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    pub fn check_empty(&mut self)->bool{
        self.stack.len()==0
    }

    pub fn check_full(&mut self)->bool{
        self.stack.len()==self.size
    }

    pub fn get_size(&mut self)->usize{
        self.size
    }

    pub fn pop(&mut self)->Option<T>{
        self.stack.pop()
    }
}