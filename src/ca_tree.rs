use crate::Table;
use std::cmp;

// a CATree port based on https://dl.acm.org/doi/10.1145/2633448.2633455

impl<K: Ord, V> CATree<K, V> {
    pub fn new() -> Self {
        CATree { root: None }
    }
}

impl Table for CATree {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut tree = CATree::new();
        tree.set(1, "one");
        tree.set(2, "two");
        tree.set(3, "three");

        assert_eq!(tree.get(&1), Some(&"one"));
        assert_eq!(tree.get(&2), Some(&"two"));
        assert_eq!(tree.get(&3), Some(&"three"));
        assert_eq!(tree.get(&4), None);
    }
}
