use anyhow::Result;

pub trait Table<K, V>
where
    K: Send + Sync,
    V: Send + Sync
{
    fn new() -> Result<Self> where Self: Sized;
    fn get(key: K) -> Result<V>;
    fn set(key: K, value: V) -> Result<()>;
}
