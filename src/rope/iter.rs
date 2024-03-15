use super::{leaf::Leaf, node::Node};


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
                Self::traverse(&internal.branches[0], nodes);
                Self::traverse(&internal.branches[1], nodes);
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
