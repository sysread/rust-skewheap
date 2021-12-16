//! A mergeable priority heap

use std::collections::VecDeque;


pub trait Item: PartialOrd + Copy {}
impl<T: PartialOrd + Copy> Item for T {}


#[derive(Debug)]
struct Node<T> {
    item:  T,
    left:  *mut Node<T>,
    right: *mut Node<T>,
}

impl<T: Item> Node<T> {
    fn new(item: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            item:  item,
            left:  std::ptr::null_mut(),
            right: std::ptr::null_mut(),
        }))
    }

    fn merge(a: *mut Self, b: *mut Self) -> *mut Self {
        if a.is_null() {
            return b
        }

        if b.is_null() {
            return a
        }

        unsafe {
            // Swap args to preserve correct ordering if a > b
            if (*a).item > (*b).item {
                std::ptr::swap(a, b);
            }

            // Build a new node from b and a's right child
            let new_left_node = Node::merge(b, (*a).right);

            // Move a's left node to the right side
            (*a).right = (*a).left;

            // Replace a's left node with the merger of b and a's right node
            (*a).left = new_left_node;

            return a;
        }
    }
}

impl<T: Item + std::fmt::Display> Node<T> {
    pub fn explain(&self, indent: usize) {
        let indent_str = format!("{:width$}", "", width=(indent * 3));

        unsafe {
            println!("{}Node: {}", indent_str, (*self).item);

            if !(*self).left.is_null() {
                println!("{}   Left:", indent_str);
                (*(*self).left).explain(indent + 2);
            }

            if !(*self).right.is_null() {
                println!("{}   Right:", indent_str);
                (*(*self).right).explain(indent + 2);
            }
        }
    }
}


/// A skew heap is an unbounded priority (min) heap. It is paramaterized by the type of item to be
/// stored in it. Items must implement PartialOrd and Clone.
#[derive(Debug)]
pub struct SkewHeap<T> {
    count: usize,
    root:  *mut Node<T>,
}

impl<T: Item> SkewHeap<T> {
    /// Returns a new SkewHeap
    pub fn new() -> Self {
        Self {
            count: 0,
            root:  std::ptr::null_mut(),
        }
    }

    /// Returns the number of items in the SkewHeap
    pub fn size(&self) -> usize {
        self.count
    }

    /// Returns true if there are no items currently in the SkewHeap
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Inserts an item into the heap and returns the new size
    pub fn put(&mut self, item: T) -> usize {
        let node = Node::new(item);

        if self.is_empty() {
            self.root = node;
        } else {
            self.root = Node::merge(self.root, node);
        }

        self.count += 1;
        self.count
    }

    /// Removes and retrieves the top item from the heap
    pub fn take(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }

        let root = self.root;
        let item;

        unsafe {
            item = (*root).item;
            self.root = Node::merge((*root).left, (*root).right);

            // free old root node by giving ownership of it to Box
            Box::from_raw(root);
        }

        self.count -= 1;
        Some(item)
    }

    /// Retrieves the top item from the heap without removing it
    pub fn peek(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }

        unsafe {
            Some((*self.root).item)
        }
    }

    /// Merge another skew heap into this one. Once merged, the other heap is destroyed.
    pub fn adopt(&mut self, mut other: SkewHeap<T>) {
        self.root = Node::merge(self.root, other.root);
        self.count += other.count;

        // self has taken possession of other's node pointers. We must remove the root pointer from
        // other and set its count to 0 in order to prevent drop() from attempting to free other's
        // root tree.
        other.root = std::ptr::null_mut();
        other.count = 0;
    }
}

impl<T: Item + std::fmt::Display> SkewHeap<T> {
    /// Prints out the entire tree structure for debugging
    pub fn explain(&self) {
        println!("SkewHeap<size={}>", self.count);

        if !self.root.is_null() {
            unsafe {
                (*self.root).explain(1)
            }
        }
    }
}

impl<T> Drop for SkewHeap<T> {
    fn drop(&mut self) {
        if !self.root.is_null() {
            let mut stack = VecDeque::from([self.root]);

            loop {
                if let Some(node) = stack.pop_front() {
                    unsafe {
                        if !(*node).left.is_null() {
                            stack.push_front((*node).left);
                        }

                        if !(*node).right.is_null() {
                            stack.push_front((*node).right);
                        }

                        Box::from_raw(node);
                    }
                }
                else {
                    break
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SkewHeap;

    #[test]
    fn test_positive_path() {
        let mut skew = SkewHeap::new();

        assert!(skew.is_empty(), "initially empty");
        assert_eq!(skew.peek(), None, "peek returns None when is_empty");
        assert_eq!(skew.take(), None, "take returns None when is_empty");

        assert_eq!(skew.put(10), 1, "put returns new size");
        assert_eq!(skew.peek(), Some(10), "peek returns top entry after put");
        assert_eq!(skew.size(), 1, "size returns expected count after put");
        assert!(!skew.is_empty(), "is_empty false after put");

        assert_eq!(skew.put(3), 2, "put returns new size");
        assert_eq!(skew.peek(), Some(3), "peek returns top entry after put");
        assert_eq!(skew.size(), 2, "size returns expected count after put");
        assert!(!skew.is_empty(), "is_empty false after put");

        assert_eq!(skew.put(15), 3, "put returns new size");
        assert_eq!(skew.peek(), Some(3), "peak returns top entry after put");
        assert_eq!(skew.size(), 3, "size returns expected count after put");
        assert!(!skew.is_empty(), "is_empty false after put");

        assert_eq!(skew.take(), Some(3), "take returns top entry");
        assert_eq!(skew.peek(), Some(10), "peek returns top entry after take");
        assert_eq!(skew.size(), 2, "size returns expected count after take");
        assert!(!skew.is_empty(), "is_empty false when > 0 entries");

        assert_eq!(skew.take(), Some(10), "take returns top entry");
        assert_eq!(skew.peek(), Some(15), "peek returns top entry after take");
        assert_eq!(skew.size(), 1, "size returns expected count after take");
        assert!(!skew.is_empty(), "is_empty false when > 0 entries");

        assert_eq!(skew.take(), Some(15), "take returns top entry");
        assert_eq!(skew.peek(), None, "peek returns None after final entry returned by take");
        assert_eq!(skew.size(), 0, "size is 0 after final entry returned by take");
        assert!(skew.is_empty(), "is_empty true after final entry returned by take");
    }

    #[test]
    fn test_merge_heaps() {
        let mut a = SkewHeap::new();
        a.put(1);
        a.put(2);
        a.put(3);

        let mut b = SkewHeap::new();
        b.put(4);
        b.put(5);
        b.put(6);

        a.adopt(b);
        assert_eq!(a.size(), 6);
        assert_eq!(a.take(), Some(1));
        assert_eq!(a.take(), Some(2));
        assert_eq!(a.take(), Some(3));
        assert_eq!(a.take(), Some(4));
        assert_eq!(a.take(), Some(5));
        assert_eq!(a.take(), Some(6));
    }
}
