use std::sync::Arc;

use super::{
    func::{insert, remove_at, Context},
    internal::Internal,
    iter::LeafIterator,
    node::Node,
};

// Rope data structure
#[derive(Debug, Clone)]
pub struct Rope {
    pub root: Arc<Node>,
    len: usize,
}

// TODO: Remove all panics
impl Rope {
    pub fn new() -> Self {
        Self {
            root: Arc::new(Node::Internal(Internal::new())),
            len: 0,
        }
    }

    fn with_root(node: Node, len: usize) -> Rope {
        Rope {
            len,
            root: Arc::new(node),
        }
    }

    // TODO: Do we really need to clone in this method?
    pub fn value(&self) -> String {
        LeafIterator::new(&self.root)
            .map(|val| val.clone())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn append(&mut self, arg: &str) {
        self.insert(self.len, arg)
    }

    // TODO: Add max node len
    // TODO: Use result
    // BUG BUG BUG BUG BUG BUG
    // BUG BUG BUG BUG BUG BUG
    // TODO: THIS SHOULD ONLY INSERTS ON FIRST INDEX!!!!
    pub fn insert(&mut self, index: usize, arg: &str) {
        let node = Arc::make_mut(&mut self.root);
        if index > self.len {
            panic!("Index out of bounds");
        }
        let context = Context::new(index, arg);
        node.do_at(context, insert);
        self.len += arg.len();
    }
    pub fn concat(self, other: Rope) -> Rope {
        let mut new_internal = Internal::new();
        new_internal.branches[0] = self.root.clone();
        new_internal.branches[1] = other.root.clone();
        new_internal.weight = self.len;
        Rope::with_root(Node::from(new_internal), self.len + other.len)
    }

    pub fn remove_at(&mut self, index: usize) {
        if self.len() == 0 {
            // Do nothing
            return;
        }
        if index > self.len() {
            panic!("Index out of bounds");
        }
        let node = Arc::make_mut(&mut self.root);
        let context = Context::new(index, "");
        node.do_at(context, remove_at);
        self.len -= 1;
    }

    pub fn rebalance(&mut self) {
        // TODO: Find better way of rebalancing this?
        // TODO: Find a way to determine a better rebalancing timing
        let collected = Node::from(self.value().as_str());
        self.root = Arc::new(collected);
    }

    // TODO: Remove. Used only for benching traversal. Add cfg?
    pub fn do_nothing_at(&mut self) {
        self.append("");
    }

    pub fn split_at(&self, index: usize) -> (Rope, Rope) {
        if index == 0 {
            return (Rope::new(), self.clone());
        }
        if index > self.len {
            panic!("Index out of bounds");
        }
        let value = self.value();
        let (left, right) = value.split_at(index as usize);
        (Rope::from(left), Rope::from(right))
    }
}

impl From<&str> for Rope {
    fn from(value: &str) -> Self {
        let root = Node::from(value);
        let len = value.len();
        Rope::with_root(root, len)
    }
}

// TODO: Add handling lines
#[cfg(test)]
mod tests {
    use super::Rope;

    #[test]
    fn create_empty_rope() {
        let rope = Rope::new();

        assert_eq!(rope.root.weight(), 0);
        assert_eq!(rope.value(), "");
    }

    #[test]
    fn create_rope_and_append() {
        let mut rope = Rope::new();

        rope.append("hello");

        assert_eq!(rope.len(), 5);
        assert_eq!(rope.value(), "hello");
    }

    #[test]
    fn concat_ropes() {
        let mut first = Rope::new();
        let mut second = Rope::new();

        first.append("hello");
        second.append(" world");

        let rope = first.concat(second);
        assert_eq!("hello world", rope.value());
    }

    #[test]
    fn add_in_the_middle() {
        let mut first = Rope::new();
        let mut second = Rope::new();

        first.append("Hello");
        second.append(" World");

        let mut rope = first.concat(second);
        assert_eq!("Hello World", rope.value());

        rope.insert(5, " beaufitul");
        assert_eq!("Hello beaufitul World", rope.value());
    }

    #[test]
    fn add_at_start() {
        let mut first = Rope::new();
        let mut second = Rope::new();

        first.append("Hello");
        second.append(" World");

        let mut rope = first.concat(second);
        assert_eq!("Hello World", rope.value());

        rope.insert(0, "Let's say ");
        assert_eq!("Let's say Hello World", rope.value());
    }

    // TODO: Verify if this clones the strings
    #[test]
    fn clone_and_add() {
        let mut first = Rope::new();
        let mut second = Rope::new();

        first.append("Hello");
        second.append(" World");

        let concatenated = first.concat(second);
        assert_eq!("Hello World", concatenated.value());

        let mut new = concatenated.clone();
        new.insert(5, " beaufitul");
        assert_ne!(new.value(), concatenated.value());
    }

    #[test]
    fn append_multiple_times() {
        let mut rope = Rope::new();

        rope.append("Hello");
        rope.append(" World");

        assert_eq!("Hello World", rope.value());
        assert_eq!(11, rope.len());
    }

    #[test]
    fn append_even_further() {
        let mut rope = Rope::new();

        rope.append("Hello");
        rope.append(" World");
        dbg!(&rope);
        rope.append("!");

        assert_eq!("Hello World!", rope.value());
    }

    #[test]
    fn remove_at_index() {
        let mut rope = Rope::new();

        rope.append("Hello");
        rope.remove_at(2);

        assert_eq!("Helo", rope.value());
    }

    #[test]
    fn remove_at_index_multiple() {
        let mut rope = Rope::new();

        rope.append("Hello");
        rope.remove_at(2);
        rope.remove_at(2);

        assert_eq!("Heo", rope.value());
    }

    #[test]
    fn remove_everything_one_by_one() {
        let mut rope = Rope::new();

        rope.append("Hello");
        rope.remove_at(4);
        rope.remove_at(3);
        rope.remove_at(2);
        rope.remove_at(1);
        rope.remove_at(0);

        assert_eq!("", rope.value());
    }

    #[test]
    fn add_and_remove() {
        let mut rope = Rope::new();

        rope.append("Hello");
        rope.remove_at(4);
        rope.remove_at(3);
        rope.remove_at(2);
        rope.remove_at(1);
        rope.remove_at(0);
        rope.append(" World");

        assert_eq!(" World", rope.value());
    }

    #[test]
    fn append_thousand_characters_to_end() {
        let mut rope = Rope::new();

        for _ in 0..1000 {
            rope.append("a");
        }

        assert_eq!(1000, rope.len());
    }

    #[test]
    fn append_thousand_words_longer_than_max_leaf_len() {
        let mut rope = Rope::new();
        let phrase = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";
        // Stack overflow without rebalance
        let times = 1000;

        for i in 1..=times {
            println!("Insert {}", i);
            rope.append(phrase);
        }

        assert_eq!(times * phrase.len(), rope.len());
    }

    #[test]
    fn another_i_guess_question_mark() {
        let lorem = "lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";
        let mut rope = Rope::new();

        let mut counter = 0;
        for _ in 0..6000 {
            counter += 1;
            println!("Insert {}", counter);
            rope.append(lorem);
            assert_eq!(counter * 124, rope.len());
        }
    }

    #[test]
    #[should_panic]
    fn adding_out_of_bounds() {
        let mut rope = Rope::new();
        rope.append("Hello");
        rope.insert(10, "whatever");
    }

    #[test]
    fn insert_in_middle() {
        let mut rope = Rope::from("Hello World");

        assert_eq!("Hello World", rope.value());

        rope.insert(5, " beaufitul");
        assert_eq!("Hello beaufitul World", rope.value());
    }

    #[test]
    fn split_rope() {
        let mut rope = Rope::from("Hello World");
        let (left, right) = rope.split_at(5);

        assert_eq!(5, left.len());
        assert_eq!(6, right.len());
        assert_eq!("Hello", left.value());
        assert_eq!(" World", right.value());
    }
}
