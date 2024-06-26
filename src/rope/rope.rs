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

    pub fn leaf_iter(&self) -> LeafIterator {
        LeafIterator::new(&self.root)
    }

    fn with_root(node: Node, len: usize) -> Rope {
        Rope {
            len,
            root: Arc::new(node),
        }
    }

    // TODO: Do we really need to clone in this method?
    // Don't clone!!!
    pub fn value(&self) -> String {
        LeafIterator::new(&self.root)
            .map(|leaf| leaf.to_string())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn append(&mut self, arg: &str) {
        self.insert(self.len, arg)
    }

    pub fn insert(&mut self, index: usize, arg: &str) {
        let node = Arc::make_mut(&mut self.root);
        if index > self.len {
            panic!("Index out of bounds");
        }
        let context = Context::new(index, arg);
        node.add_at(context, insert);
        self.len += arg.chars().count();
    }

    pub fn concat(self, other: Rope) -> Rope {
        let mut new_internal = Internal::new();
        new_internal.branches[0] = self.root;
        new_internal.branches[1] = other.root;
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
        node.remove_at(context, remove_at);
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

        let u_idx: usize = value.chars().take(index).map(|c| c.len_utf8()).sum();

        let (left, right) = value.split_at(u_idx);
        (Rope::from(left), Rope::from(right))
    }
}

impl From<&str> for Rope {
    fn from(value: &str) -> Self {
        let root = Node::from(value);

        let len = value.chars().count();
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

        first.append("śhello");
        second.append(" worldś");

        let rope = first.concat(second);
        assert_eq!(13, rope.len());
        assert_eq!("śhello worldś", rope.value());
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
        let rope = Rope::from("Hello World");
        let (left, right) = rope.split_at(5);

        assert_eq!(5, left.len());
        assert_eq!(6, right.len());
        assert_eq!("Hello", left.value());
        assert_eq!(" World", right.value());
    }

    #[test]
    fn utf8_append() {
        let mut rope = Rope::from("Hello");

        rope.append(", Worlduśś!");

        assert_eq!("Hello, Worlduśś!", rope.value());
    }

    #[test]
    fn utf8_insert_in_middle() {
        let mut rope = Rope::from("Hello");

        rope.insert(2, "Ś");

        assert_eq!("HeŚllo", rope.value());
    }

    #[test]
    fn utf8_insert_after_utf8_character() {
        let mut rope = Rope::from("HeŚlo");

        rope.insert(3, "l");

        assert_eq!("HeŚllo", rope.value());
    }

    #[test]
    fn utf8_into_empty_rope() {
        let mut rope = Rope::new();

        rope.append("ą");

        assert_eq!("ą", rope.value());
    }

    #[test]
    fn remove_utf8() {
        let mut rope = Rope::from("Hśello");

        rope.remove_at(1);

        assert_eq!("Hello", rope.value());
    }

    #[test]
    fn remove_at_beginning_utf8() {
        let mut rope = Rope::from("ŚHello");

        rope.remove_at(0);

        assert_eq!("Hello", rope.value());
    }

    #[test]
    fn remove_at_end_utf8() {
        let mut rope = Rope::from("Helloś");

        rope.remove_at(5);

        assert_eq!("Hello", rope.value());
    }

    #[test]
    fn split_after_utf8() {
        let rope = Rope::from("Helloś World");

        let (left, right) = rope.split_at(5);

        assert_eq!("Hello", left.value());
        assert_eq!("ś World", right.value());
    }

    #[test]
    fn test_failing_split() {
        let mut rope = Rope::new();
        rope.append("śśś");

        let (left, right) = rope.split_at(3);

        assert_eq!("śśś", left.value());
        assert_eq!("", right.value());
    }

    #[test]
    fn concat_and_split() {
        let a = Rope::from("asdfś");
        let b = Rope::from("asdf");

        let c = a.concat(b);
        assert_eq!("asdfśasdf", c.value());

        let (left, right) = c.split_at(5);
        assert_eq!("asdfś", left.value());
        assert_eq!("asdf", right.value());
    }

    #[test]
    fn rope_from_add_delete() {
        let mut rope = Rope::from("Witam");

        rope.insert(0, "c");
        rope.insert(1, "c");
        rope.insert(2, "c");

        assert_eq!("cccWitam", rope.value());

        rope.remove_at(0);
        assert_eq!("ccWitam", rope.value());

        rope.remove_at(0);
        assert_eq!("cWitam", rope.value());

        rope.remove_at(0);
        assert_eq!("Witam", rope.value());
    }

    #[test]
    fn prepend() {
        let mut rope = Rope::from("Witam");

        rope.insert(0, "N");
        rope.insert(1, "c");
        rope.insert(2, "c");

        assert_eq!("NccWitam", rope.value());
    }
}
