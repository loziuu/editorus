use std::{io::BufRead, thread::current};

use super::{
    internal::Internal,
    node::{Node, Weight},
};

// We assume that one page is 4096 bytes long.
// Vec pointer + last_char_index + vec len + vec capacity
pub const TOTAL_BYTES: usize = 1024;

//pub const INTERNAL_SIZE: usize = std::mem::size_of::<usize>() + std::mem::size_of::<[u8; 2]>();
pub const INTERNAL_SIZE: usize = 0;

pub const LEAF_SIZE: usize = std::mem::size_of::<Vec<u8>>() + std::mem::size_of::<usize>();

pub const NODE_SIZE: usize = LEAF_SIZE + INTERNAL_SIZE;

// We are trying to fit internal in TOTAL_BYTES.
//pub const MAX_LEAF_LEN: usize = (TOTAL_BYTES - NODE_SIZE) / 2;
pub const MAX_LEAF_LEN: usize = TOTAL_BYTES - NODE_SIZE;

// TODO: This should be immutable eventually... reallly?
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Leaf {
    pub(super) val: Vec<u8>,
    pub(super) last_char_index: usize,
}

impl Leaf {
    pub fn new(val: Vec<u8>, last_char_index: usize) -> Self {
        Self {
            val,
            last_char_index,
        }
    }

    pub fn available_space(&self) -> usize {
        MAX_LEAF_LEN - self.last_char_index
    }

    pub fn get_char_bytes(&self) -> &[u8] {
        &self.val[..self.last_char_index]
    }

    pub fn byte_position_of_char_at(&self, index: usize) -> usize {
        if self.last_char_index == 0 {
            return 0;
        }

        unsafe {
            std::str::from_utf8_unchecked(&self.val[..self.last_char_index])
                .chars()
                .take(index)
                .map(|c| c.len_utf8())
                .sum()
        }
    }

    pub(crate) fn split_at_char(&mut self, index: usize) -> (Leaf, Leaf) {
        let split_point = self.byte_position_of_char_at(index);
        let mut current_leaf_val = std::mem::take(&mut self.val);

        let (actual_data, _) = current_leaf_val.split_at_mut(self.last_char_index);

        let (left, right) = actual_data.split_at(split_point);

        (Leaf::from(left), Leaf::from(right))
    }

    // TODO: Now it's 2x O(n). Make it single pass.
    // This splits the leaf, to: (Leaf(0,a), Leaf(b, last_char_index))
    pub(crate) fn remove_char_at(&mut self, index: usize) -> (Leaf, Leaf) {
        let a = self.byte_position_of_char_at(index);
        let b = self.byte_position_of_char_at(index + 1);

        let mut current_leaf_val = std::mem::take(&mut self.val);
        let (actual_data, _) = current_leaf_val.split_at_mut(self.last_char_index);
        let (left, right) = actual_data.split_at(a);
        let (_, right) = right.split_at(b - a);

        (Leaf::from(left), Leaf::from(right))
    }

    pub(crate) fn remove_char_at_node(&mut self, index: usize) -> Node {
        println!("Removing char at: {}", index);
        if index == 0 {
            println!("Removing first char");
            let idx = self.byte_position_of_char_at(1);
            return Node::Leaf(Leaf::from(&self.val[idx..self.last_char_index]));
        }
        println!("Removing further.");
        let (left, right) = self.remove_char_at(index);
        Node::Internal(Internal::with_branches(Node::Leaf(left), Node::Leaf(right)))
    }

    pub(crate) fn prepend(&mut self, buffer: &str) {
        let mut new_node_val = [0; MAX_LEAF_LEN];

        let val = buffer.as_bytes();
        let start = val.len();
        new_node_val[..start].copy_from_slice(val);

        new_node_val[start..start + self.last_char_index]
            .copy_from_slice(&self.val[..self.last_char_index]);

        self.val = new_node_val.to_vec();
        self.last_char_index += start;
    }
}

impl Weight for Leaf {
    fn weight(&self) -> usize {
        unsafe {
            std::str::from_utf8_unchecked(self.get_char_bytes())
                .chars()
                .count()
        }
    }
}

// This should be used only by Node::from. It's not public API.
// TODO: Hide this from public API. Only node.rs should be able to use it.
impl From<&str> for Leaf {
    fn from(value: &str) -> Self {
        // TODO: This should probably split and return Node - maybe leaf, maybe internal. So
        // move it to node.rs implementation.
        // ^ it's actually done in Node::from
        Leaf::from(value.as_bytes())
    }
}

impl From<&[u8]> for Leaf {
    fn from(value: &[u8]) -> Self {
        if value.len() > MAX_LEAF_LEN {
            panic!("Leaf cannot be longer than {} bytes", MAX_LEAF_LEN);
        }
        let last_index = value.len();

        let mut vec = vec![0; MAX_LEAF_LEN];
        vec[..last_index].copy_from_slice(value);

        Self {
            val: vec,
            last_char_index: last_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rope::node::Node;

    use super::Leaf;

    #[test]
    fn test_leaf_from_str() {
        let leaf = super::Leaf::from("Hello, world!");

        assert_eq!("Hello, world!".to_string(), leaf_to_str(&leaf));
        assert_eq!(leaf.last_char_index, 13);
    }

    #[test]
    fn remove_at_first_char() {
        let mut leaf = super::Leaf::from("Hello, world!");

        let node = leaf.remove_char_at_node(0);

        if let Node::Leaf(node) = node {
            assert_eq!("ello, world!".to_string(), leaf_to_str(&node));
        }
    }

    fn leaf_to_str(leaf: &Leaf) -> String {
        std::str::from_utf8(&leaf.val[..leaf.last_char_index])
            .unwrap()
            .to_string()
    }
}
