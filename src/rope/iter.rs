use super::Node;

pub(crate) struct LeafIterator<'a> {
    nodes: Vec<&'a Node>,
    index: usize,
}

impl<'a> LeafIterator<'a> {
    pub fn new(node: &'a Node) -> Self {
        // TODO: Maybe try and store somewhere the count of leafs?
        let mut nodes = vec![];
        Self::traverse(node, &mut nodes);
        Self { nodes, index: 0 }
    }

    fn traverse(node: &'a Node, nodes: &mut Vec<&'a Node>) {
        if node.left.is_none() && node.right.is_none() {
            nodes.push(node);
        }

        if let Some(left) = &node.left {
            Self::traverse(left, nodes);
        }

        if let Some(right) = &node.right {
            Self::traverse(right, nodes);
        }
    }
}

impl<'a> Iterator for LeafIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.nodes.len() {
            None
        } else {
            let node = self.nodes[self.index];
            self.index += 1;
            Some(node)
        }
    }
}
