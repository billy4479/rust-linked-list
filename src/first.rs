mod first {
    use std::mem;

    enum Link<T> {
        Empty,
        More(Box<Node<T>>),
    }

    pub struct List<T> {
        head: Link<T>,
    }

    struct Node<T> {
        value: T,
        next: Link<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List::<T> { head: Link::Empty }
        }

        pub fn push(&mut self, input: T) {
            let new_node = Box::new(Node {
                value: input,
                next: mem::replace(&mut self.head, Link::Empty),
            });

            self.head = Link::More(new_node)
        }

        // pub fn append(&mut self, input: T) {
        //     let new_node = Box::new(Node {
        //         value: input,
        //         next: Link::Empty,
        //     });
        // }

        pub fn pop(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, Link::Empty) {
                Link::Empty => None,
                Link::More(v) => {
                    self.head = v.next;
                    Some(v.value)
                }
            }
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut current = mem::replace(&mut self.head, Link::Empty);

            while let Link::More(mut node) = current {
                current = mem::replace(&mut node.next, Link::Empty);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::List;

        #[test]
        fn basics() {
            let mut list = List::<i32>::new();

            assert_eq!(list.pop(), None);

            list.push(2);
            list.push(5);
            list.push(1);

            assert_eq!(list.pop(), Some(1));
            assert_eq!(list.pop(), Some(5));

            list.push(1);
            assert_eq!(list.pop(), Some(1));
            assert_eq!(list.pop(), Some(2));

            assert_eq!(list.pop(), None);
        }
    }
}
