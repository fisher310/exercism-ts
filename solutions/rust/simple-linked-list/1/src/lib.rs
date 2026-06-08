pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            element,
            next: None,
        }
    }

    fn new_with_next(element: T, next: Option<Box<Node<T>>>) -> Self {
        Node { element, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut curr = self.head.as_ref();
        let mut s = 0;
        while curr.is_some() {
            s += 1;
            curr = curr.unwrap().next.as_ref();
        }
        s
    }

    pub fn push(&mut self, _element: T) {
        let node = Some(Box::new(Node::new_with_next(_element, self.head.take())));
        self.head = node;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else {
            let mut h = self.head.take().unwrap();
            self.head = h.next.take();
            Some(h.element)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(n) = self.head.as_ref() {
            Some(&n.element)
        } else {
            None
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut prev = None;
        let mut curr = self.head.take();
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        self.head = prev;
        self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for el in _iter {
            list.push(el);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut res = Vec::new();
        let mut curr = _linked_list.head;
        while let Some(node) = curr.take() {
            res.insert(0, node.element);
            curr = node.next;
        }
        res
    }
}
