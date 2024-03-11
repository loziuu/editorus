use super::node::Weight;

pub const MAX_LEAF_LEN: usize = 64000;

// TODO: This should be immutable eventually
#[derive(Clone, Debug)]
pub(super) struct Leaf {
    // TODO: Shouldn't this be on heap?
    pub(super) val: Vec<char>,
    pub(super) last_char_index: usize,
}

 impl Leaf {
    pub fn available_space(&self) -> usize {
        MAX_LEAF_LEN - self.last_char_index
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
        if value.len() > MAX_LEAF_LEN {
            panic!("Leaf cannot be longer than {}", MAX_LEAF_LEN);
        }
        let last_index = value.len();
        // We can unwrap it as we know that it's not longer than MAX_LEAF_LEN

        let mut val: Vec<char> = value.chars().take(MAX_LEAF_LEN).collect();
        let mut alloc = ['\0'; MAX_LEAF_LEN];

        let taken = std::mem::take(&mut val);
        alloc[..taken.len()].copy_from_slice(&taken);

        Self {
            val: alloc.to_vec(),
            last_char_index: last_index,
        }
    }
}

impl From<&[char]> for Leaf {
    fn from(value: &[char]) -> Self {
        if value.len() > MAX_LEAF_LEN {
            panic!("Leaf cannot be longer than {}", MAX_LEAF_LEN);
        }
        let mut alloc = ['\0'; MAX_LEAF_LEN];
        alloc[..value.len()].copy_from_slice(&value);
        Self {
            val: alloc.to_vec(),
            last_char_index: value.len(),
        }
    }
}
