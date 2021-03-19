use super::*;

impl<T> List<T> {
    pub fn push(&mut self, input: T) {
        let new_node = Box::new(Node {
            value: input,
            next: self.head.take(),
        });

        self.head = Link::Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|v| {
            self.head = v.next;
            v.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|v| &v.value)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|v| &mut v.value)
    }
}
