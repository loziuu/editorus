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

    pub(crate) fn weight_offset() {

    }
}

// TODO: Shouldn't leaf be immutable?
// TODO: Split leaf once weight > MAX_LEAF_NODE
// TODO: Change it after leaf
pub(crate) fn insert(context: Context, leaf: &mut Leaf) -> NodeResult {
    // We are appedning here
    if context.index == leaf.weight() {
        let remaining_space = MAX_LEAF_LEN - context.index;

        if remaining_space == 0 {
            let right = Node::from(context.buffer);

            let taken_val = std::mem::take(&mut leaf.val);
            let new_leaf = Leaf::new(taken_val, leaf.last_char_index);

            let new_internal = Node::from(Internal::with_branches(Node::from(new_leaf), right));
            return NodeResult::NewNode(new_internal);
        }

        // TODO: Merge this case with one above (remaining_space == 0)
        if remaining_space < context.buffer.as_bytes().len() {
            let (left, right) = context.buffer.split_at(remaining_space);
            let mut new_internal = Internal::new();

            // TODO: Make if leaf method?
            let left_chars = left.as_bytes();
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

        println!("Appending to: {:?}", leaf.get_char_bytes());
        let val = context.buffer.as_bytes();
        let len = val.len();
        leaf.val[leaf.last_char_index..leaf.last_char_index + len].copy_from_slice(&val);
        leaf.last_char_index += len;

        println!("Leaf: {:?}", leaf.get_char_bytes());

        NodeResult::EditedInPlace
    } else {
        if context.index > leaf.last_char_index {
            panic!("Index out of bounds");
        }
        let (mut left, right) = leaf.split_at_char(context.index);
        insert(context, &mut left);
        NodeResult::NewNode(Node::from(Internal::with_branches(
            Node::from(left),
            Node::from(right),
        )))
    }
}

// TODO: Check if we could even consume the leaf here?
pub(crate) fn remove_at(context: Context, leaf: &mut Leaf) -> NodeResult {
    println!("{:?}", leaf.get_char_bytes());
    if leaf.last_char_index == 0 {
        println!("Skipping"); 
        // Do nothing, as there is nothing to remove.
        NodeResult::EditedInPlace
    } else if context.index == leaf.last_char_index {
        println!("Let's see...");
        // We are at the end so just modify last char index
        // No so fast amigo!
        leaf.last_char_index -= 1;
        NodeResult::EditedInPlace
    } else {
        let node = leaf.remove_char_at_node(context.index);
        NodeResult::NewNode(node)
    }
}
