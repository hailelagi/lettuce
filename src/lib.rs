use std::{cmp, ops::Range};

use anyhow::Result;
// use futures::Future;
// futures_io::AsyncRead;
//  futures_core::stream::Stream;

pub trait OrderedSet<K, V>
where
    K: Send + Sync + cmp::Ord,
    V: Send + Sync,
{
    fn new() -> Result<Self> where  Self: Sized;
    fn insert(key: K) -> Result<V>;
    fn get(key: K, value: V) -> Result<()>;
    fn scan(range: Range<K>) -> Result<()>;
    fn delete(key: K) -> Result<()>;
}
