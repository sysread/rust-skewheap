//! A mergeable priority heap
use std::collections::VecDeque;

/// Parameterizes the SkewHeap. Items stored in the heap are prioritized in ascending order.
pub trait Item: Ord + Copy {}
impl<T: Ord + Copy> Item for T {}


type BoxedNode<T> = Box<Node<T>>;
type Tree<T> = Option<BoxedNode<T>>;


#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T> {
    item:  T,
    left:  Tree<T>,
    right: Tree<T>,
}

impl<T: Item> Node<T> {
    fn new(item: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Some(Box::new(Node{ item, left, right }))
    }

    fn merge (a: Tree<T>, b: Tree<T>) -> Tree<T> {
        let mut queue: VecDeque<BoxedNode<T>> = VecDeque::new();
        let mut trees: VecDeque<BoxedNode<T>> = VecDeque::new();

        if let Some(a) = a {
            queue.push_back(a);
        }

        if let Some(b) = b {
            queue.push_back(b);
        }

        // Cut right subtrees from each path
        while queue.len() > 0 {
            if let Some(mut node) = queue.pop_front() {
                // Remove the right node and add it to the queue, if present.
                if let Some(right) = node.right {
                    queue.push_back(right);
                    node.right = None;
                }

                // Add the node to the list of cut nodes
                trees.push_back(node);
            }
        }

        // Sort the collected subtrees
        trees.make_contiguous().sort();

        // Reduce right by popping off the back of the list, using the final/ultimate node as our
        // accumulator, merging the ultimate node into the penultimate node until there is only
        // one left.
        while trees.len() > 1 {
            if let Some(ult) = trees.pop_back() {
                if let Some(mut penult) = trees.pop_back() {
                    // Swap the left and right if the penultimate node has a left subtree. Its
                    // right subtree has already been cut.
                    if let Some(left) = penult.left {
                        penult.right = Some(left);
                    }

                    // Set the penultimate's left child to be the previous ultimate
                    penult.left = Some(ult);

                    // Now make the penultimate be the ultimate node in the list
                    trees.push_back(penult);
                }
            }
        }

        trees.pop_front()
    }
}


/// A skew heap is an unbounded priority (min) heap. It is paramaterized by the type of item to be
/// stored in it. Items must implement PartialOrd and Clone.
pub struct SkewHeap<T: Item> {
    size: u64,
    root: Tree<T>,
}

impl<T: Item> SkewHeap<T> {
    /// Returns a new SkewHeap
    pub fn new() -> SkewHeap<T> {
        SkewHeap{
            size: 0,
            root: None,
        }
    }

    /// Returns the number of items in the SkewHeap
    pub fn size(&self) -> u64 {
        return self.size
    }

    /// Returns true if there are no items currently in the SkewHeap
    pub fn is_empty(&self) -> bool {
        return self.size == 0
    }

    /// Inserts an item into the heap and returns the new size
    pub fn put(&mut self, item: T) -> u64 {
        self.root = match &self.root {
            None    => Node::new(item, None, None),
            Some(r) => Node::merge(Some(r.clone()), Node::new(item, None, None)),
        };

        self.size += 1;

        return self.size
    }

    /// Removes and retrieves the top item from the heap
    pub fn take(&mut self) -> Option<T> {
        return match &self.root {
            None    => None,
            Some(r) => {
                self.size -= 1;
                let item = r.item;
                self.root = Node::merge(r.left.clone(), r.right.clone());
                Some(item)
            }
        }
    }

    /// Retrieves the top item from the heap without removing it
    pub fn peek(&self) -> Option<T> {
        return match &self.root {
            None    => None,
            Some(r) => Some(r.item),
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
