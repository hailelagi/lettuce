use crate::Table;
// use std::cmp::Ordering;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    height: i32,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

#[derive(Debug)]
pub struct WAVLTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> WAVLTree<K, V> {
    pub fn new() -> Self {
        WAVLTree { root: None }
    }
}

impl Table for WAVLTree {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut tree = WAVLTree::new();
        tree.set(1, "one");
        tree.set(2, "two");
        tree.set(3, "three");

        assert_eq!(tree.get(&1), Some(&"one"));
        assert_eq!(tree.get(&2), Some(&"two"));
        assert_eq!(tree.get(&3), Some(&"three"));
        assert_eq!(tree.get(&4), None);
    }
}
