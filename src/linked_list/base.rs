use super::*;

impl<T> List<T> {
    pub fn new() -> Self {
        List::<T> { head: Link::None }
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
