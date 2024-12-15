use std::{any::Any, cmp::Ord};

#[derive(Debug, Clone)]
pub struct WAVLTree<K, V> {
    root: Link<K, V>,
    height: usize
}

type Link<K,V> = Option<Box<Node<K, V>>>;

unsafe impl<K: Send, V: Send> Send for WAVLTree<K, V> {}
unsafe impl<K: Sync, V: Sync> Sync for WAVLTree<K, V> {}

#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
    height: i32,
}

impl<K: Ord, V> WAVLTree<K, V> {
    pub fn new() -> Self {
        WAVLTree { root: None, height: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<()>
    where
        K: Ord,
    {
        if let Some(n) = self.root.as_mut() {
            n.insert(key, value)
        } else {
            self.root = Some(Box::new(Node {
                key: key,
                value: value,
                height: 0,
                left: None,
                right: None,
            }));
            Some(())
        }
    }

    pub fn get(&mut self, key: K) -> Option<&V>
    where
        K: Ord,
        V: Any,
    {
        if let Some(n) = self.root.as_mut() {
            if n.key == key {
                Some(&n.value)
            } else {
                //n.search(key)
                None
            }
        } else {
            None
        }
    }
}

impl<K: Ord, V> Node<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<()>
    where
        K: Ord,
    {
        if self.key == key {
            self.value = value;
            Some(())
        } else if self.key < key {
            match &mut self.left {
                Some(n) => n.insert(key, value),
                None => {
                    self.left = Some(Box::new(Node {
                        key: key,
                        value: value,
                        height: 0,
                        left: None,
                        right: None,
                    }));

                    Some(())
                }
            }
        } else {
            match self.right.as_mut() {
                Some(n) => n.insert(key, value),
                None => None,
            }
        }
    }

    pub fn search(&mut self, key: K, value: V) -> Option<()>
    where 
       K: Ord,
       {
        None
       }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut tree = WAVLTree::new();
        tree.insert(1, "one");
        tree.insert(2, "two");
        tree.insert(3, "three");

        assert_eq!(tree.get(1), Some(&"one"));
        assert_eq!(tree.get(2), Some(&"two"));
        assert_eq!(tree.get(3), Some(&"three"));
        assert_eq!(tree.get(4), None);
    }
}
