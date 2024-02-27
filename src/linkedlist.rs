use std::ptr::null;

struct Node<T> {
    data: T,
    next: *const Node<T>,
}

//Allocating some node in the heap
impl<T> Node<T> {
    pub fn new(data: T, next_ptr: *const Node<T>) -> *mut Node<T> {
        let node = Box::new(Node {
            data,
            next: next_ptr,
        });

        Box::into_raw(node)
    }
}

struct LinkedList<T> {
    head: *const Node<T>,
    tail: *const Node<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: null(),
            tail: null(),
        }
    }

    pub fn push_front(&mut self, data: T) {
        unsafe {
            if self.head.is_null() {
                let node = Node::new(data, null());
                self.head = node;
                self.tail = node;
            } else {
                let node = Node::new(data, self.head);
                self.head = node;
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        unsafe {
            let node = Node::new(data, null());
            if self.tail.is_null() {
                self.tail = node;
                self.head = node;
            } else {
                self.tail.read().next = node;
                self.tail = node
            }
        }
    }

    pub fn get_head(&self) -> *const Node<T> {
        unsafe { self.head }
    }

    pub fn get_tail(&self) -> *const Node<T> {
        unsafe { self.tail }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_points_to_nothing() {
        let linkedlist = LinkedList::<String>::new();
        assert_eq!(linkedlist.get_head().is_null(), true);
    }

    #[test]
    fn it_should_points_to_some_data_in_the_tail() {
        let mut linkedlist = LinkedList::<String>::new();
        linkedlist.push_back("Mawfyy".to_string());
        assert_eq!(linkedlist.get_tail().is_null(), false);
    }

    #[test]
    fn it_should_points_to_some_data_in_the_head() {
        let mut linkedlist = LinkedList::<String>::new();
        linkedlist.push_front("Mawfyy".to_string());
        assert_eq!(linkedlist.get_head().is_null(), false);
    }
}
