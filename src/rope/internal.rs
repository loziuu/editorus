use std::sync::Arc;

use super::node::{Node, Weight};

#[derive(Clone, Debug)]
pub(super) struct Internal {
    pub(super) weight: usize,
    pub(super) left: Option<Arc<Node>>,
    pub(super) right: Option<Arc<Node>>,
}

impl Internal {
    pub fn new() -> Self {
        Self {
            weight: 0,
            left: None,
            right: None,
        }
    }

    pub fn with_branches(left: Node, right: Node) -> Self {
        Self {
            weight: left.weight(),
            left: Some(Arc::new(left)),
            right: Some(Arc::new(right)),
        }
    }

    pub fn with_branches_and_weight(left: Node, right: Node, weight: usize) -> Self {
        Self {
            weight,
            left: Some(Arc::new(left)),
            right: Some(Arc::new(right)),
        }
    }
}

impl Weight for Internal {
    fn weight(&self) -> usize {
        self.weight
    }
}
