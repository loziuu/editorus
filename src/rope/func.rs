use std::sync::Arc;

use super::{
    internal::Internal,
    leaf::{Leaf, MAX_LEAF_LEN},
    node::{Node, Weight},
};

//  TODO: Better name please
#[derive(Debug)]
pub enum NodeResult {
    NewNode(Node),
    EditedInPlace,
}

// TODO: Change this name as "Context" is very vague
// TODO: Refactor this to be more functional I guess, as it is pretty much only feasible for
// addition
#[derive(Debug)]
pub(crate) struct Context<'a> {
    pub index: usize,
    pub buffer: &'a str,
}

impl<'a> Context<'a> {
    pub(crate) fn new(index: usize, buffer: &'a str) -> Self {
        Self { index, buffer }
    }
}

// TODO: Shouldn't leaf be immutable?
// TODO: Split leaf once weight > MAX_LEAF_NODE
// TODO: Change it after leaf
pub(crate) fn insert(context: Context, leaf: &mut Leaf) -> NodeResult {
    // We are appedning here
    if context.index == leaf.last_char_index {
        let remaining_space = MAX_LEAF_LEN - context.index;

        if remaining_space == 0 {
            let right = Node::from(context.buffer);

            let vec = std::mem::take(&mut leaf.val);
            let new_leaf = Leaf::new(vec, leaf.last_char_index);

            let new_internal = Node::from(Internal::with_branches(Node::from(new_leaf), right));
            return NodeResult::NewNode(new_internal);
        }

        if remaining_space < context.buffer.len() {
            // No need to split, as we can't fit anything here.
            // TODO: What if context.buffer itself is bigger than MAX_LEAF_LEN?

            //            panic!("Implement the case when we can fit some of it.");
            let (left, right) = context.buffer.split_at(remaining_space);
            let mut new_internal = Internal::new();

            // TODO: Make if leaf method?
            let left_chars = left.as_bytes();
            // TODO: This is not optimal. We should be able to do this without allocation.
            leaf.val[leaf.last_char_index..leaf.last_char_index + remaining_space]
                .copy_from_slice(&left_chars);
            leaf.last_char_index += remaining_space;

            new_internal.weight = MAX_LEAF_LEN;

            let vec = std::mem::take(&mut leaf.val);
            let new_leaf = Leaf::new(vec, leaf.last_char_index);

            new_internal.branches[0] = Arc::new(Node::from(new_leaf));
            new_internal.branches[1] = Arc::new(Node::from(right));
            return NodeResult::NewNode(Node::from(new_internal));
        }

        let val = context.buffer.as_bytes();
        let len = val.len();
        leaf.val[leaf.last_char_index..leaf.last_char_index + len].copy_from_slice(&val);
        leaf.last_char_index += len;
        NodeResult::EditedInPlace
    } else {
        if context.index > leaf.last_char_index {
            panic!("Index out of bounds");
        }
        let mut current_leaf_val = std::mem::take(&mut leaf.val);
        let (actual_data, _) = current_leaf_val.split_at_mut(leaf.last_char_index);

        let (left, right) = actual_data.split_at(context.index);
        let mut left_leaf = Leaf::from(left);
        let right_leaf = Leaf::from(right);
        insert(context, &mut left_leaf);
        NodeResult::NewNode(Node::from(Internal::with_branches(
            Node::from(left_leaf),
            Node::from(right_leaf),
        )))
    }
}

// TODO: Check if we could even consume the leaf here?
pub(crate) fn remove_at(context: Context, leaf: &mut Leaf) -> NodeResult {
    if leaf.last_char_index == 0 {
        // Do nothing, as there is nothing to remove.
        NodeResult::EditedInPlace
    } else if context.index == leaf.last_char_index {
        // We are at the end so just modify last char index
        leaf.last_char_index -= 1;
        NodeResult::EditedInPlace
    } else {
        // Split and move index
        let (left, right) = leaf.val.split_at(context.index);
        let mut right_leaf = Leaf::from(&right[1..]);
        right_leaf.last_char_index = leaf.last_char_index - context.index - 1;

        let mut left_leaf = Leaf::from(left);
        left_leaf.last_char_index = leaf.last_char_index - right_leaf.last_char_index - 1;

        let mut new_internal = Internal::new();
        new_internal.weight = left_leaf.weight();
        new_internal.branches[0] = Arc::new(Node::from(Leaf::from(left_leaf)));
        new_internal.branches[1] = Arc::new(Node::from(Leaf::from(right_leaf)));

        NodeResult::NewNode(Node::from(new_internal))
    }
}
