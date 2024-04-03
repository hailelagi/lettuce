use std::cmp;

use anyhow::Result;
// use futures::Future;

pub trait Table<K, V>
where
    K: Send + Sync + cmp::Ord,
    V: Send + Sync,
{
    fn new() -> Result<Self>
    where
        Self: Sized;

    // futures_io::AsyncRead;
    //  futures_core::stream::Stream;
    fn get(key: K) -> Result<V>;
    fn set(key: K, value: V) -> Result<()>;
}
