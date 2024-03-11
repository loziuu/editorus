use std::sync::Arc;

use super::{
    func::{Context, NodeResult},
    internal::Internal,
    leaf::{Leaf, MAX_LEAF_LEN},
};

pub(crate) trait Weight {
    fn weight(&self) -> usize;
}

#[derive(Clone, Debug)]
pub(crate) enum Node {
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

    pub(crate) fn do_at<F>(&mut self, ctx: Context, f: F) -> NodeResult
    where
        F: Fn(Context, &mut Leaf) -> NodeResult,
    {
        match self {
            Node::Leaf(node) => f(ctx, node),
            Node::Internal(node) => {
                let weight = node.weight;
                // Can we simplify this somehow?
                if let Some(left) = node.left.as_mut() {
                    if weight > ctx.index {
                        // TODO: Should I bump the weight here? Modification will be on left so...
                        match Arc::make_mut(left).do_at(ctx, f) {
                            NodeResult::NewNode(new_node) => {
                                node.left = Some(Arc::new(new_node));
                                NodeResult::EditedInPlace
                            }
                            _ => NodeResult::EditedInPlace,
                        }
                    } else if let Some(right) = node.right.as_mut() {
                        let context = Context::new(ctx.index - weight, ctx.buffer);
                        match Arc::make_mut(right).do_at(context, f) {
                            NodeResult::NewNode(new_node) => {
                                node.right = Some(Arc::new(new_node));
                                NodeResult::EditedInPlace
                            }
                            _ => NodeResult::EditedInPlace,
                        }
                    } else {
                        // TODO: Do it only if index < total tope len
                        node.right = Some(Arc::new(Node::from(Leaf::from(ctx.buffer.as_str()))));
                        NodeResult::EditedInPlace
                    }
                } else {
                    // Nothing on left so we should insert leaf there.
                    node.left = Some(Arc::new(Node::from(Leaf::from(ctx.buffer.as_str()))));
                    // Is it good place for addition? What if we are doing removal?
                    node.weight += ctx.buffer.len();
                    NodeResult::EditedInPlace
                }
                // I tu bedzie call? Match po NodeResult?
            }
        }
    }
}

impl From<&str> for Node {
    fn from(arg: &str) -> Self {
        if arg.len() > MAX_LEAF_LEN {
            // TODO: Split
            // Maybe we should split in half? Splitting in half would result in more balanced tree
            let (left, right) = arg.split_at(arg.len() / 2);
            let left = Node::from(left);
            let right = Node::from(right);
            Node::Internal(Internal::with_branches(left, right))
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
