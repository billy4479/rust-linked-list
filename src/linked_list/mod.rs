pub mod iterators;

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::<T> { head: Link::None }
    }

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

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();

        while let Link::Some(mut node) = current {
            current = node.next.take();
        }
    }
}
