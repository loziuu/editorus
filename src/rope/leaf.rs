use std::{char::MAX, io::Bytes};

use super::node::Weight;

// We assume that one page is 4096 bytes long.
// Vec pointer + last_char_index + vec len + vec capacity
pub const TOTAL_BYTES: usize = 4096;
pub const LEAF_POINTERS_SIZE: usize = std::mem::size_of::<usize>() * 3;
pub const MAX_LEAF_LEN: usize = TOTAL_BYTES - (2 * LEAF_POINTERS_SIZE);

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

    pub fn val(&self) -> &[u8] {
        &self.val[..self.last_char_index]
    }
}

impl Weight for Leaf {
    fn weight(&self) -> usize {
        self.last_char_index
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
        let mut vec = value.to_vec();
        vec.resize(MAX_LEAF_LEN, 0);

        Self {
            val: vec,
            last_char_index: last_index,
        }
    }
}
