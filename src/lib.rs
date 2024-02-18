use anyhow::Result;

pub trait Table<K, V>
where
    T: Static + Send + Sync,
{
    fn new() -> Result<Self>;
    fn get(key: K) -> Result<V>;
    fn set(key: K, value: V) -> Result<()>;
}
