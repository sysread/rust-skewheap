//! A mergeable priority heap

use std::collections::VecDeque;

pub trait Item: PartialOrd + Copy {}
impl<T: PartialOrd + Copy> Item for T {}

type Index  = usize;
type Handle = Option<Index>;

struct Node<T> {
    item:  Option<T>,
    left:  Handle,
    right: Handle,
}

impl<T: Item> Node<T> {
    fn new(item: Option<T>) -> Node<T> {
        Node{
            item:  item,
            left:  None,
            right: None,
        }
    }
}

/// A skew heap is an unbounded priority (min) heap. It is paramaterized by the type of item to be
/// stored in it. Items must implement PartialOrd and Clone.
pub struct SkewHeap<T> {
    count: usize,
    root:  Handle,
    nodes: Vec<Node<T>>,
    freed: VecDeque<Index>,
}

impl<T: Item> SkewHeap<T> {
    /// Returns a new SkewHeap
    pub fn new() -> Self {
        Self {
            count: 0,
            root:  None,
            nodes: Vec::new(),
            freed: VecDeque::new(),
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
        let node = self.alloc_node(item);
        self.root = self.merge(self.root, node);
        self.count += 1;
        self.count
    }

    /// Removes and retrieves the top item from the heap
    pub fn take(&mut self) -> Option<T> {
        if self.count == 0 {
            return None
        }

        if let Some(root) = self.root {
            let item  = self.nodes[root].item;
            let left  = self.nodes[root].left;
            let right = self.nodes[root].right;

            self.free_node(root);

            self.count -= 1;
            self.root = self.merge(left, right);

            item
        } else {
            None
        }
    }

    /// Retrieves the top item from the heap without removing it
    pub fn peek(&self) -> Option<T> {
        match self.root {
            None    => None,
            Some(n) => self.nodes[n].item,
        }
    }

    fn merge(&mut self, a: Handle, b: Handle) -> Handle {
        match (a, b) {
            (None,    None)                                               => None,
            (Some(a), None)                                               => Some(a),
            (None,    Some(b))                                            => Some(b),
            (Some(a), Some(b)) if self.nodes[a].item > self.nodes[b].item => self.merge(Some(b), Some(a)),
            (Some(a), Some(b))                                            => {
                let tmp = self.nodes[a].right;
                self.nodes[a].right = self.nodes[a].left;
                self.nodes[a].left = self.merge(Some(b), tmp);
                Some(a)
            },
        }
    }

    fn alloc_node(&mut self, item: T) -> Handle {
        if let Some(idx) = self.freed.pop_front() {
            self.nodes[idx].item = Some(item);
            Some(idx)
        } else {
            self.nodes.push(Node::new(Some(item)));
            Some(self.nodes.len() - 1)
        }
    }

    fn free_node(&mut self, idx: Index) {
        self.nodes[idx].left  = None;
        self.nodes[idx].right = None;
        self.nodes[idx].item  = None;
        self.freed.push_back(idx);
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
}
