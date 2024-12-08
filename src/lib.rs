pub mod template;
use std::fmt::Debug;
use std::hash::Hash;

use heapless::FnvIndexMap as HeaplessHashMap;
use heapless::Vec as HeaplessVec;

// Use this file to add helper functions and additional modules.

pub fn into_group_map_heapless<const M: usize, const N: usize, I, K, V>(
    iter: I,
) -> HeaplessHashMap<K, HeaplessVec<V, M>, N>
where
    V: Clone + Debug,
    I: Iterator<Item = (K, V)>,
    K: Hash + Eq + Debug,
{
    let mut lookup: HeaplessHashMap<K, HeaplessVec<V, M>, N> = HeaplessHashMap::new();

    iter.for_each(|(key, val)| {
        if let Some(vec) = lookup.get_mut(&key) {
            vec.push(val).unwrap();
        } else {
            let mut vec = HeaplessVec::new();
            vec.push(val).unwrap();
            lookup.insert(key, vec).unwrap();
        }
    });

    lookup
}
