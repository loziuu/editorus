use self::iter::LeafIterator;

mod iter;

// Rope data structure
struct Rope {
    root: Box<Node>,
    len: usize,
}

// TODO: Remove all panics
impl Rope {
    fn new() -> Self {
        Self {
            root: Box::new(Node::new()),
            len: 0,
        }
    }

    fn value(&self) -> String {
        LeafIterator::new(&self.root)
            .map(|node| String::from_utf8(node.val.clone()).unwrap())
            .collect()
    }

    fn append(&mut self, arg: &str) {
        self.insert(arg, self.len)
    }

    fn insert(&mut self, arg: &str, index: usize) {
        if index > self.len {
            panic!("Index out of bounds");
        }
        todo!()
    }
}


struct Node {
    weight: usize,
    val: Vec<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Node {
        Self {
            weight: 0,
            val: vec![],
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rope;

    #[test]
    fn create_empty_rope() {
        let rope = Rope::new();

        assert_eq!(rope.root.weight, 0);
        assert_eq!(rope.value(), "");
    }

    #[test]
    fn create_rope_and_append() {
        let mut rope = Rope::new();

        rope.append("hello");

        assert_eq!(rope.root.weight, 5);
        assert_eq!(rope.value(), "hello");
    }
}
