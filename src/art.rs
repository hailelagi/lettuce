// art.rs

use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    Leaf(String),
    Inner(HashMap<char, Box<Node>>),
}

#[derive(Debug)]
pub struct AdaptiveRadixTree {
    root: Node,
}

impl AdaptiveRadixTree {
    pub fn new() -> Self {
        AdaptiveRadixTree {
            root: Node::Inner(HashMap::new()),
        }
    }

    pub fn insert(&mut self, key: &str, value: String) {
        let mut current_node = &mut self.root;
        for ch in key.chars() {
            match current_node {
                Node::Inner(ref mut children) => {
                    current_node = children.entry(ch).or_insert_with(|| Box::new(Node::Inner(HashMap::new())));
                }
                Node::Leaf(_) => unreachable!(),
            }
        }
        *current_node = Node::Leaf(value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        let mut current_node = &self.root;
        for ch in key.chars() {
            match current_node {
                Node::Inner(ref children) => {
                    current_node = children.get(&ch)?;
                }
                Node::Leaf(_) => return None,
            }
        }
        match current_node {
            Node::Leaf(ref value) => Some(value),
            Node::Inner(_) => None,
        }
    }
}
