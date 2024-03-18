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

    #[inline]
    pub(crate) fn do_at<F>(&mut self, ctx: Context, f: F) -> NodeResult
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
                    Arc::make_mut(left).do_at(ctx, f)
                } else {
                    let right = &mut node.branches[1];
                    let context = Context::new(ctx.index - weight, ctx.buffer);
                    Arc::make_mut(right).do_at(context, f)
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
            let (left, right) = arg.split_at(arg.len() / 2); 
            let left_node = Node::from(left);
            let right_node = Node::from(right);
            let r = Node::Internal(Internal::with_branches_and_weight(
                left_node,
                right_node,
                left.len(),
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
