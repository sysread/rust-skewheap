pub trait Item: PartialOrd + Clone {}
impl<T: PartialOrd + Clone> Item for T {}


type Tree<'a, T> = Option<Box<Node<'a, T>>>;


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

    pub fn get(&mut self) -> Option<&'a T> {
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

    pub fn peak(&self) -> Option<&'a T> {
        return match &self.root {
            None    => None,
            Some(r) => Some(r.item),
        }
    }
}


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

#[cfg(test)]
mod tests {
    use super::SkewHeap;

    #[test]
    fn test_ordering() {
        let mut skew = SkewHeap::new();

        assert!(skew.is_empty());
        assert_eq!(skew.peak(), None);

        skew.put(&10);
        assert_eq!(skew.peak(), Some(&10));

        skew.put(&3);
        assert_eq!(skew.peak(), Some(&3));

        skew.put(&15);
        assert_eq!(skew.peak(), Some(&3));

        assert_eq!(skew.size, 3);
        assert!(!skew.is_empty());

        assert_eq!(skew.peak(), Some(&3));
        assert_eq!(skew.get(), Some(&3));
        assert_eq!(skew.size, 2);
        assert!(!skew.is_empty());

        assert_eq!(skew.peak(), Some(&10));
        assert_eq!(skew.get(), Some(&10));
        assert_eq!(skew.size, 1);
        assert!(!skew.is_empty());

        assert_eq!(skew.peak(), Some(&15));
        assert_eq!(skew.get(), Some(&15));
        assert_eq!(skew.size, 0);
        assert!(skew.is_empty());
    }
}
