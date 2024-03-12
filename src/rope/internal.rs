use std::sync::Arc;

use super::node::{Node, Weight};

// TODO: Make this at max 4kb
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Internal {
    pub(super) weight: usize,
    pub(super) branches: [Option<Arc<Node>>; 2],
}

impl Internal {
    pub fn new() -> Self {
        Self {
            weight: 0,
            branches: [None, None],
        }
    }

    pub fn with_branches(left: Node, right: Node) -> Self {
        Self {
            weight: left.weight(),
            branches: [Some(Arc::new(left)), Some(Arc::new(right))],
        }
    }

    pub fn with_branches_and_weight(left: Node, right: Node, weight: usize) -> Self {
        Self {
            weight,
            branches: [Some(Arc::new(left)), Some(Arc::new(right))],
        }
    }
}

impl Weight for Internal {
    fn weight(&self) -> usize {
        self.weight
    }
}
