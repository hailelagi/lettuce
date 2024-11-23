use std::{cmp, ops::Range};

use anyhow::Result;

pub mod ca_tree;
pub mod avl;
pub mod art;

pub trait OrderedSet<K, V>
where
    K: Send + Sync + cmp::Ord,
    V: Send + Sync,
{
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn get(key: K, value: V) -> Result<()>;
    fn insert(key: K) -> Result<V>;
    fn range(range: Range<K>) -> Result<()>;
    fn bulk_insert(range: Range<K>) -> Result<()>;
    fn delete(key: K) -> Result<()>;
}
