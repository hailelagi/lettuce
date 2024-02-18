use hashbrown::HashMap;
use parking_lot::RawRwLock;

pub struct Map {
    store: HashMap,
    locks: [RawRwLock; 8],
    global: RawRwLock,
}

impl Table for Map {}
