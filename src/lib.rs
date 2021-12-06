pub struct SkewHeap {
    size: u64,
    root:  Tree,
}

impl SkewHeap {
    pub fn new() -> SkewHeap {
        SkewHeap{
            size: 0,
            root:  None,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0
    }

    pub fn put(&mut self, item: Item) -> u64 {
        self.root = match &self.root {
            Some(r) => Node::merge(&Some(r.clone()), &Node::new(item, None, None)),
            None    => Node::new(item, None, None)
        };

        self.size += 1;

        return self.size
    }

    pub fn get(&mut self) -> Option<Item> {
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

    pub fn peak(&self) -> Option<Item> {
        return match &self.root {
            None    => None,
            Some(r) => Some(r.item),
        }
    }
}

type Item = i64;
type Tree = Option<Box<Node>>;

#[derive(Clone)]
struct Node {
    item:  Item,
    left:  Tree,
    right: Tree,
}

impl Node {
    fn new(item: Item, left: Tree, right: Tree) -> Tree {
        Some(Box::new(Node{ item: item, left: left, right: right }))
    }

    fn merge<'a>(a: &'a Tree, b: &'a Tree) -> Tree {
        match (a,b) {
            (None,    None)                       => None,
            (Some(a), None)                       => Some(a.clone()),
            (None,    Some(b))                    => Some(b.clone()),
            (Some(a), Some(b)) if a.item > b.item => Node::merge(&Some(b.clone()), &Some(a.clone())),
            (Some(a), Some(b))                    => Node::new(a.item, Node::merge(&a.right, &Some(b.clone())).clone(), a.left.clone()),
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

        skew.put(10);
        assert_eq!(skew.peak(), Some(10));

        skew.put(3);
        assert_eq!(skew.peak(), Some(3));

        skew.put(15);
        assert_eq!(skew.peak(), Some(3));

        assert_eq!(skew.size, 3);
        assert!(!skew.is_empty());

        assert_eq!(skew.peak(), Some(3));
        assert_eq!(skew.get(), Some(3));
        assert_eq!(skew.size, 2);
        assert!(!skew.is_empty());

        assert_eq!(skew.peak(), Some(10));
        assert_eq!(skew.get(), Some(10));
        assert_eq!(skew.size, 1);
        assert!(!skew.is_empty());

        assert_eq!(skew.peak(), Some(15));
        assert_eq!(skew.get(), Some(15));
        assert_eq!(skew.size, 0);
        assert!(skew.is_empty());
    }
}
