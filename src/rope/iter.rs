use super::{leaf::Leaf, Node};

// TODO: Get it to not clone?
pub(super) struct LeafIterator<'a> {
    nodes: Vec<&'a Leaf>,
    index: usize,
}

impl<'a> LeafIterator<'a> {
    pub fn new(node: &'a Node) -> Self {
        // TODO: Maybe try and store somewhere the count of leafs?
        let mut nodes = vec![];
        Self::traverse(node, &mut nodes);
        Self { nodes, index: 0 }
    }

    fn traverse(node: &'a Node, nodes: &mut Vec<&'a Leaf>) {
        match node {
            Node::Leaf(node) => {
                nodes.push(&node);
            }
            Node::Internal(internal) => {
                if let Some(left) = &internal.branches[0] {
                    Self::traverse(left, nodes);
                }

                if let Some(right) = &internal.branches[1] {
                    Self::traverse(right, nodes);
                }
            }
        }
    }
}

impl<'a> Iterator for LeafIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.nodes.len() {
            None
        } else {
            let node = self.nodes[self.index];
            self.index += 1;
            let val = std::str::from_utf8(&node.val());
            Some(val.unwrap().to_string())
        }
    }
}
