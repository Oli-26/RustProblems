pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Copy> LinkedList<T> {
    // Create a new empty list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    fn find_tail_mut(&mut self) -> Option<&mut Box<Node<T>>> {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.next.is_none() {
                return Some(node);
            }
            current = &mut node.next;
        }
        None
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });
    
        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }
    
        if let Some(tail) = self.find_tail_mut() {
            tail.next = Some(new_node);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
    
        if self.head.as_ref().unwrap().next.is_none() {
            return self.pop_front();
        }
    
        let mut prev = &mut self.head;
    
        while let Some(ref mut node) = prev.as_mut().unwrap().next {
            if node.next.is_none() {
                let removed_node = prev.as_mut().unwrap().next.take();
                return removed_node.map(|node| node.data);
            }
            prev = &mut prev.as_mut().unwrap().next;
        }
    
        None
    }
    

    pub fn insert(&mut self, index: usize, data: T) {
        let mut current = &mut self.head;

        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }

        let new_node = Node {
            data,
            next: current.take(),
        };

        *current = Some(Box::new(new_node));
    }

    pub fn remove (&mut self, index: usize){
        let mut current = &mut self.head;

        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }        

        let removed_node = current.take();
        *current = removed_node.unwrap().next;
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        self.head = prev;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_insert() {
        let mut list = LinkedList::new();
        list.insert(0, 1);
        list.insert(1, 2);
        list.insert(1, 3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
    }

    #[test]
    fn test_remove() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.remove(1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_reverse() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.reverse();
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
    }
}
pub fn main(){

}
