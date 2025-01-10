

pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { data: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> { // will need to be unwrapped when used
        Some(self.data.remove(0))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_full(&self) -> bool {
        self.size() == self.data.len()
    }

    pub fn get_first(&self) -> Option<&T> { // will need to be unwrapped when used
        self.data.first()
    }

    pub fn get_last(&self) -> Option<&T> { // will need to be unwrapped when used
        self.data.last()
    }
}