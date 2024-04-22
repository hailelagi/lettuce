use std::cmp;

// a rust CATree port based on:
// * https://doi.org/10.1016/j.jpdc.2017.11.007
// * https://dl.acm.org/doi/10.1145/2633448.2633455

// reference implementation:
// https://github.com/erlang/otp/blob/629597747844b45a79458bde84cf1d7440aad46e/erts/emulator/beam/erl_db_catree.h
// https://github.com/erlang/otp/blob/629597747844b45a79458bde84cf1d7440aad46e/erts/emulator/beam/erl_db_catree.c

impl<K: Ord, V> CATree<K, V> {
    pub fn new() -> Self {
        CATree { root: None }
    }
}

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
