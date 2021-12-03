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
        let node: Tree = Node::new(item, None, None);

        self.root = match self.root {
            None    => node,
            Some(_) => Node::merge(self.root, node),
        };

        self.items += 1;

        return self.items
    }
}

type Item = i64;
type Tree = Option<Box<Node>>;

struct Node {
    item:  Item,
    left:  Tree,
    right: Tree,
}

impl Node {
    fn new(item: Item, left: Tree, right: Tree) -> Tree {
        Some(Box::new(Node{ item: item, left: left, right: right }))
    }

    fn merge(a: Tree, b: Tree) -> Tree {
        let pair = (a, b);

        match pair {
            (None,    None)                       => None,
            (Some(a), None)                       => Some(a),
            (None,    Some(b))                    => Some(b),
            (Some(a), Some(b)) if a.item > b.item => Node::merge(Some(b), Some(a)),
            (Some(a), Some(b))                    => Node::new(a.item, Node::merge(a.right, Some(b)), a.left),
        }
    }
}
