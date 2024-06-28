use std::cmp;
use std::collections::BTreeMap;
use std::sync::atomic::AtomicUsize;

use parking_lot::RwLock;

// a rust CATree port based on:
// * https://doi.org/10.1016/j.jpdc.2017.11.007
// * https://dl.acm.org/doi/10.1145/2633448.2633455

// A CA Tree implements an ordered set, with runtime adaptive locking.
// Three layers:
// 1. routing nodes
// 2. base nodes
// 3. sequential ordered datastructure(AVL, Redblack Tree, Treap, etc)
// in this case a B-Tree

// reference implementation:
// https://github.com/erlang/otp/blob/629597747844b45a79458bde84cf1d7440aad46e/erts/emulator/beam/erl_db_catree.h
// https://github.com/erlang/otp/blob/629597747844b45a79458bde84cf1d7440aad46e/erts/emulator/beam/erl_db_catree.c

pub enum LockState {
    LockFailureContribution,
    LockSuccessContribution,
    LockGravityContribution,
    LockGravityPattern,
    LockMoreThanOneContribution,
    HighContentionLimit,
    LowContentionLimit,
    MaxRouteNodeLayerHeight,
    LockLowNoContributionLimit,
    LockHighNoContributionLimit,
}

pub struct CATree<K, V> {
    internal_node: Node<K>,
    base_node: BTreeMap<K, V>,
}

struct Node<K> {
    key: K,
    // less than or equal to routing node's key
    left: Option<Box<Node<K>>>,
    // greater than routing node's key
    right: Option<Box<Node<K>>>,
    lock: StatisticLock<K>
}

// #[repr(C)]
struct StatisticLock<T> {
    mu: RwLock<T>,
    flag: LockState,
    statistic: AtomicUsize,
}

unsafe impl<K: Send + Sync, V: Send + Sync> Send for CATree<K, V> {}
unsafe impl<K: Send + Sync, V: Send + Sync> Sync for CATree<K, V> {}

impl<K: cmp::Ord, V> CATree<K, V> {
    pub fn new() -> Self {
        CATree {
            base_node: BTreeMap::new(),
            internal_node: Node {
                key: 0,
                value: 0,
                left: None,
                right: None,
                height: 1,
            },
        }
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

        assert_eq!(tree.base_node.get(&1), Some(&"one"));
        assert_eq!(tree.base_node.get(&2), Some(&"two"));
        assert_eq!(tree.base_node.get(&3), Some(&"three"));
        assert_eq!(tree.base_node.get(&4), None);
    }
}
