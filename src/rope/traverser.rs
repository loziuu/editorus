use std::sync::Arc;

use super::{node::Node, Rope};

#[derive(Clone, Debug)]
pub struct Traverser {
    curr_node: Arc<Node>,
    level: usize,
    prev: Option<Arc<Traverser>>,
}

impl Traverser {
    pub fn new(rope: &Rope) -> Self {
        Self {
            curr_node: rope.root.clone(),
            level: 0,
            prev: None,
        }
    }

    pub fn go_back(&self) -> Arc<Traverser> {
        let arc = &self.prev;
        match arc {
            Some(prev) => {
                println!("Going back");
                prev.clone()
            },
            None => Arc::new(self.clone()),
        }
    }

    pub fn go_right(&self) -> Arc<Traverser> {
        match &self.curr_node.as_ref() {
            Node::Leaf(_) => Arc::new(self.clone()),
            Node::Internal(internal) => match &internal.branches[1] {
                Some(right) => {
                    println!("Going right");
                    Arc::new(Traverser {
                        curr_node: right.clone(),
                        level: self.level + 1,
                        prev: Some(Arc::new(self.clone())),
                    })
                }
                None => {
                    println!("No right node");
                    Arc::new(self.clone())
                }
            },
        }
    }

    pub fn go_left(&self) -> Arc<Traverser> {
        match &self.curr_node.as_ref() {
            Node::Leaf(_) => Arc::new(self.clone()),
            Node::Internal(internal) => match &internal.branches[0] {
                Some(right) => {
                    println!("Going left");
                    Arc::new(Traverser {
                        curr_node: right.clone(),
                        level: self.level + 1,
                        prev: Some(Arc::new(self.clone())),
                    })
                }
                None => {
                    println!("No left node");
                    Arc::new(self.clone())
                }
            },
        }
    }

    pub fn current(&self) {
        match &self.curr_node.as_ref() {
            Node::Leaf(leaf) => {
                println!("Current node is leaf on level {}. [Last index at: <{}>, Val: <{:?}>]", self.level, leaf.last_char_index, leaf.val);
            }
            Node::Internal(_) => {
                println!(
                    "Current node is internal on level {}. Weight: {}.",
                    self.level,
                    self.curr_node.weight()
                );
            }
        }
    }
}
