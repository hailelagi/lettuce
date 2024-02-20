use hashbrown::HashMap;
use parking_lot::RawRwLock;

pub struct Map<K, V> {
    store: HashMap<K, V>,
    locks: [RawRwLock; 8],
    global: RawRwLock,
}

impl Table for Map {}
