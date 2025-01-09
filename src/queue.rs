

struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.data.push(item);
    }

    fn dequeue(&mut self) -> Option<T> { // will need to be unwrapped when used
        Some(self.data.remove(0))
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn size(&self) -> usize {
        self.data.len()
    }

    fn is_full(&self) -> bool {
        self.size() == self.data.len()
    }

    fn get_first(&self) -> Option<&T> { // will need to be unwrapped when used
        self.data.first()
    }

    fn get_last(&self) -> Option<&T> { // will need to be unwrapped when used
        self.data.last()
    }
}