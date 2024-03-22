use std::sync::Arc;

use super::{
    func::{Context, NodeResult},
    internal::Internal,
    leaf::{Leaf, MAX_LEAF_LEN},
};

pub trait Weight {
    fn weight(&self) -> usize;
}

#[derive(Clone, Debug)]
#[repr(u8, C)]
pub enum Node {
    Leaf(Leaf),
    Internal(Internal),
}

impl Node {
    pub fn weight(&self) -> usize {
        match self {
            Node::Leaf(leaf) => leaf.weight(),
            Node::Internal(internal) => internal.weight(),
        }
    }

    pub(crate) fn add_at<F>(&mut self, mut ctx: Context, f: F) -> NodeResult
    where
        F: Fn(Context, &mut Leaf) -> NodeResult,
    {
        match self {
            Node::Leaf(node) => {
                match f(ctx, node) {
                    NodeResult::NewNode(new_node) => {
                        // TODO: Should if actually be std::mem::swap?
                        *self = new_node;
                        NodeResult::EditedInPlace
                    }
                    NodeResult::EditedInPlace => NodeResult::EditedInPlace,
                }
            }
            Node::Internal(node) => {
                let weight = node.weight;
                if weight > ctx.index {
                    let left = &mut node.branches[0];
                    node.weight += ctx.buffer.len();
                    Arc::make_mut(left).add_at(ctx, f)
                } else {
                    let right = &mut node.branches[1];
                    ctx.index -= weight;
                    Arc::make_mut(right).add_at(ctx, f)
                }
            }
        }
    }

    pub(crate) fn remove_at<F>(&mut self, mut ctx: Context, f: F) -> NodeResult
    where
        F: Fn(Context, &mut Leaf) -> NodeResult,
    {
        match self {
            Node::Leaf(node) => {
                match f(ctx, node) {
                    NodeResult::NewNode(new_node) => {
                        // TODO: Should if actually be std::mem::swap?
                        *self = new_node;
                        NodeResult::EditedInPlace
                    }
                    NodeResult::EditedInPlace => NodeResult::EditedInPlace,
                }
            }
            Node::Internal(node) => {
                let weight = node.weight;
                if weight > ctx.index {
                    let left = &mut node.branches[0];
                    node.weight -= 1; 
                    Arc::make_mut(left).add_at(ctx, f)
                } else {
                    let right = &mut node.branches[1];
                    ctx.index -= weight;
                    Arc::make_mut(right).add_at(ctx, f)
                }
            }
        }
    }
}

impl From<&str> for Node {
    fn from(arg: &str) -> Self {
        if arg.len() > MAX_LEAF_LEN {
            // Split at half OR split at max leaf len
            //let (left, right) = arg.split_at(MAX_LEAF_LEN);
            // THIS IS WRONG.

            let n = arg.chars().count();

            let (left, right) = arg.split_at(n / 2);

            let left_node = Node::from(left);
            let right_node = Node::from(right);
            let r = Node::Internal(Internal::with_branches_and_weight(
                left_node,
                right_node,
                n / 2,
            ));
            r
        } else {
            Node::Leaf(Leaf::from(arg))
        }
    }
}

impl From<Internal> for Node {
    fn from(val: Internal) -> Self {
        Node::Internal(val)
    }
}

impl From<Leaf> for Node {
    fn from(val: Leaf) -> Self {
        Node::Leaf(val)
    }
}
