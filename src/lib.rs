struct SkewHeap {
    items: u64,
    root:  Tree,
}

impl SkewHeap {
    fn new() -> SkewHeap {
        SkewHeap{
            items: 0,
            root:  None,
        }
    }

    fn put(&mut self, item: Item) -> u64 {
        self.root = match &self.root {
            Some(r) => Node::merge(&Some(r.clone()), &Node::new(item, None, None)),
            None    => Node::new(item, None, None)
        };

        self.items += 1;

        return self.items
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
