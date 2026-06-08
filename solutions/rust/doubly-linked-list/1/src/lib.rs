use std::ptr::NonNull;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

#[derive(Debug, Clone, Default)]
struct Node<T> {
    data: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

#[derive(Debug, Clone, Default)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

pub struct Cursor<'a, T> {
    node: Option<NonNull<Node<T>>>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    node: Option<&'a NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.head,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.tail,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            node: self.head.as_ref(),
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(ptr) = self.head {
            let node = unsafe { Box::from_raw(ptr.as_ptr()) };
            self.head = node.next;
            drop(node)
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let node = unsafe { self.node?.as_mut() };
        Some(&mut node.data)
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        self.node = unsafe { self.node.take()?.as_ref().next };
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        self.node = unsafe { self.node.take()?.as_ref().prev };
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let node = unsafe { Box::from_raw(self.node.take()?.as_ptr()) };

        match node.next {
            Some(mut ptr) => unsafe {
                ptr.as_mut().prev = node.prev;
            },
            None => {
                self.list.tail = node.prev;
            }
        }

        match node.prev {
            Some(mut ptr) => unsafe {
                ptr.as_mut().next = node.next;
            },
            None => {
                self.list.head = node.next;
            }
        }

        self.node = node.next.or(node.prev);
        let Node { data, .. } = *node;
        Some(data)
    }

    pub fn insert_after(&mut self, _element: T) {
        let next = self.node.and_then(|ptr| unsafe { ptr.as_ref().next });
        let node_ptr = NonNull::new(Box::leak(Box::new(Node {
            data: _element,
            next,
            prev: self.node,
        })));
        match next {
            Some(mut ptr) => unsafe {
                ptr.as_mut().prev = node_ptr;
            },
            None => {
                self.list.tail = node_ptr;
            }
        }

        match self.node {
            Some(mut ptr) => unsafe {
                ptr.as_mut().next = node_ptr;
            },
            None => {
                self.list.head = node_ptr;
            }
        }
    }

    pub fn insert_before(&mut self, _element: T) {
        let prev = self.node.and_then(|ptr| unsafe { ptr.as_ref().prev });
        let node_ptr = NonNull::new(Box::leak(Box::new(Node {
            data: _element,
            prev,
            next: self.node,
        })));
        match prev {
            Some(mut ptr) => unsafe {
                ptr.as_mut().next = node_ptr;
            },
            None => {
                self.list.head = node_ptr;
            }
        }

        match self.node {
            Some(mut ptr) => unsafe {
                ptr.as_mut().prev = node_ptr;
            },
            None => self.list.tail = node_ptr,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let node = unsafe { self.node?.as_ref() };
        self.node = node.next.as_ref();
        Some(&node.data)
    }
}
