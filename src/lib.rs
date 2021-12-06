pub trait Item: PartialOrd + Clone {}
impl<T: PartialOrd + Clone> Item for T {}


type Tree<'a, T> = Option<Box<Node<'a, T>>>;


#[derive(Clone)]
struct Node<'a, T: 'a> {
    item:  &'a T,
    left:  Tree<'a, T>,
    right: Tree<'a, T>,
}

impl<'a, T: Item> Node<'a, T> {
    fn new(item: &'a T, left: Tree<'a, T>, right: Tree<'a, T>) -> Tree<'a, T> {
        Some(Box::new(Node{ item, left, right }))
    }

    fn merge<'b>(a: &'b Tree<'a, T>, b: &'b Tree<'a, T>) -> Tree<'a, T> {
        match (a, b) {
            (None,    None)                       => None,
            (Some(a), None)                       => Some(a.clone()),
            (None,    Some(b))                    => Some(b.clone()),
            (Some(a), Some(b)) if a.item > b.item => Node::merge(&Some(b.clone()), &Some(a.clone())),
            (Some(a), Some(b))                    => Node::new(a.item, Node::merge(&a.right, &Some(b.clone())), a.left.clone()),
        }
    }
}


pub struct SkewHeap<'a, T: Item> {
    size: u64,
    root: Tree<'a, T>,
}

impl<'a, T: Item> SkewHeap<'a, T> {
    pub fn new() -> SkewHeap<'a, T> {
        SkewHeap{
            size: 0,
            root: None,
        }
    }

    pub fn size(&self) -> u64 {
        return self.size
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0
    }

    pub fn put(&mut self, item: &'a T) -> u64 {
        self.root = match &self.root {
            Some(r) => Node::merge(&Some(r.clone()), &Node::new(item, None, None)),
            None    => Node::new(item, None, None)
        };

        self.size += 1;

        return self.size
    }

    pub fn take(&mut self) -> Option<&'a T> {
        return match &self.root {
            None    => None,
            Some(r) => {
                self.size -= 1;
                let item = r.item;
                self.root = Node::merge(&r.left, &r.right);
                Some(item)
            }
        }
    }

    pub fn peek(&self) -> Option<&'a T> {
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

        assert_eq!(skew.put(&10), 1, "put returns new size");
        assert_eq!(skew.peek(), Some(&10), "peek returns top entry after put");
        assert_eq!(skew.size(), 1, "size returns expected count after put");
        assert!(!skew.is_empty(), "is_empty false after put");

        assert_eq!(skew.put(&3), 2, "put returns new size");
        assert_eq!(skew.peek(), Some(&3), "peek returns top entry after put");
        assert_eq!(skew.size(), 2, "size returns expected count after put");
        assert!(!skew.is_empty(), "is_empty false after put");

        assert_eq!(skew.put(&15), 3, "put returns new size");
        assert_eq!(skew.peek(), Some(&3), "peak returns top entry after put");
        assert_eq!(skew.size(), 3, "size returns expected count after put");
        assert!(!skew.is_empty(), "is_empty false after put");

        assert_eq!(skew.take(), Some(&3), "take returns top entry");
        assert_eq!(skew.peek(), Some(&10), "peek returns top entry after take");
        assert_eq!(skew.size(), 2, "size returns expected count after take");
        assert!(!skew.is_empty(), "is_empty false when > 0 entries");

        assert_eq!(skew.take(), Some(&10), "take returns top entry");
        assert_eq!(skew.peek(), Some(&15), "peek returns top entry after take");
        assert_eq!(skew.size(), 1, "size returns expected count after take");
        assert!(!skew.is_empty(), "is_empty false when > 0 entries");

        assert_eq!(skew.take(), Some(&15), "take returns top entry");
        assert_eq!(skew.peek(), None, "peek returns None after final entry returned by take");
        assert_eq!(skew.size(), 0, "size is 0 after final entry returned by take");
        assert!(skew.is_empty(), "is_empty true after final entry returned by take");
    }
}
