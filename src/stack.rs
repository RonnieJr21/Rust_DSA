

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> { // will need to be unwrapped when used
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn size(&self) -> usize {
        self.data.len()
    }

    fn get_last(&self) -> Option<&T> { // will need to be unwrapped when used
        self.data.last()
    }
}