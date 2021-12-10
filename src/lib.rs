//! A mergeable priority heap

use std::collections::VecDeque;

pub trait Item: PartialOrd + Copy {}
impl<T: PartialOrd + Copy> Item for T {}

type Index  = usize;
type Handle = Option<Index>;

#[derive(Debug)]
struct Node<T> {
    item:   Option<T>,
    left:   Handle,
    right:  Handle,
    parent: Handle,
}

impl<T: Item> Node<T> {
    fn new(item: Option<T>) -> Node<T> {
        Node{
            item:   item,
            left:   None,
            right:  None,
            parent: None,
        }
    }
}

impl<T: Item> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.item.eq(&other.item)
    }
}

impl<T: Item> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.item.partial_cmp(&other.item)
    }
}

/// A skew heap is an unbounded priority (min) heap. It is paramaterized by the type of item to be
/// stored in it. Items must implement PartialOrd and Clone.
#[derive(Debug)]
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
        if let Some(root) = self.root {
            let item = self.nodes[root].item;
            self.count -= 1;
            self.root = self.merge(self.nodes[root].left, self.nodes[root].right);
            self.free_node(root);

            if self.needs_defrag() {
                self.defrag();
            }

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
            (None,    None)                                     => None,
            (Some(a), None)                                     => Some(a),
            (None,    Some(b))                                  => Some(b),
            (Some(a), Some(b)) if self.nodes[a] > self.nodes[b] => self.merge(Some(b), Some(a)),
            (Some(a), Some(b))                                  => {
                // Build a new node from b and a's right child
                let new_left_node = self.merge(Some(b), self.nodes[a].right);

                // Move a's left node to the right side
                self.nodes[a].right = self.nodes[a].left;

                // Replace a's left node with the merger of b and a's right node
                self.nodes[a].left = new_left_node;

                // Set the parent of the newlb merged left node to be a
                if let Some(new_left_node_idx) = new_left_node {
                    self.nodes[new_left_node_idx].parent = Some(a);
                }

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
        self.nodes[idx].item = None;
        self.freed.push_back(idx);
    }

    fn realloc_node(&mut self, from: Index) -> Handle {
        // Reorder freed indices to move lower indices to the front
        self.freed.make_contiguous().sort();

        // First free slot is further back in nodes than our subject node's current index
        if let Some(first) = self.freed.front() {
            if *first > from {
                return None;
            }
        }

        if let Some(to) = self.freed.pop_front() {
            // Copy from's data info to's memory
            self.nodes[to].left   = self.nodes[from].left;
            self.nodes[to].right  = self.nodes[from].right;
            self.nodes[to].parent = self.nodes[from].parent;
            self.nodes[to].item   = self.nodes[from].item;

            // Update the parent's child link
            if let Some(parent_id) = self.nodes[to].parent {
                if self.nodes[parent_id].left == Some(from) {
                    self.nodes[parent_id].left = Some(to);
                } else if self.nodes[parent_id].right == Some(from) {
                    self.nodes[parent_id].right = Some(to);
                }
            }

            // Update the left child's parent link
            if let Some(left_id) = self.nodes[to].left {
                self.nodes[left_id].parent = Some(to);
            }

            // Update the right child's parent link
            if let Some(right_id) = self.nodes[to].right {
                self.nodes[right_id].parent = Some(to);
            }

            // Clear the old from and add it back to the pot
            self.free_node(from);

            Some(to)
        } else {
            None
        }
    }

    fn needs_defrag(&mut self) -> bool {
        // at least 100 items and > 90% of them are freed
        self.nodes.len() >= 100 && self.freed.len() > (90 * self.nodes.len() / 100)
    }

    fn defrag(&mut self) {
        // Walk backwards over the allocated node list
        let mut i = self.nodes.len() - 1;

        loop {
            if !self.needs_defrag() {
                break;
            }

            // Some(item) means it's allocated
            if let Some(_) = self.nodes[i].item {
                // Move it to the next index in self.freed. None means it ran out of free indices.
                if let Some(new_index) = self.realloc_node(i) {
                    // If the current index is our root node, update self.root.
                    if self.root == Some(i) {
                        self.root = Some(new_index);
                    }
                } else {
                    break;
                }
            }

            if i > 0 {
                i -= 1;
            } else {
                break;
            }
        }

        // Now that all nodes have been relocated to the front of self.nodes, prune back the empty
        // slots and clear freed.
        if i < self.nodes.len() - 1 { // at least one freed slot was used
            self.nodes.truncate(self.nodes.len() - self.freed.len());
            self.freed.clear();
        }
    }
}

impl<T: Item + std::fmt::Display> SkewHeap<T> {
    /// Prints out the entire tree structure for debugging
    pub fn explain(&self) {
        println!("SkewHeap<size={}>", self.count);

        if let Some(root) = self.root {
            self._explain(root, 1);
        }
    }

    fn _explain(&self, node: Index, indent: usize) {
        let indent_str = format!("{:width$}", "", width=(indent * 3));

        if let Some(value) = self.nodes[node].item {
            println!("{}Node: {}", indent_str, value);

            if let Some(left) = self.nodes[node].left {
                println!("{}   Left:", indent_str);
                self._explain(left, indent + 2);
            }

            if let Some(right) = self.nodes[node].right {
                println!("{}   Right:", indent_str);
                self._explain(right, indent + 2);
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
}
