use hashbrown::HashMap;
use parking_lot::RawRwLock;

pub struct Map<K, V>
where
    K: Send + Sync + Sized + cmp::Ordering,
    V: Send + Sync,
{
    store: HashMap<K, V>,
    locks: [RawRwLock; 8],
    global: RawRwLock,
}
