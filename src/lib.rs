pub mod template;
use std::fmt::Debug;
use std::hash::Hash;

use heapless::FnvIndexMap as HeaplessHashMap;
use heapless::Vec as HeaplessVec;

// Use this file to add helper functions and additional modules.

#[derive(Debug)]
pub enum GroupMapError<V, K> {
    VecInsertError(V),
    HashMapInsertError(K),
}

pub fn into_group_map_heapless<const M: usize, const N: usize, I, K, V>(
    iter: I,
) -> Result<HeaplessHashMap<K, HeaplessVec<V, M>, N>, GroupMapError<V, K>>
where
    V: Clone + Debug,
    I: Iterator<Item = (K, V)>,
    K: Hash + Eq + Debug,
{
    let mut lookup: HeaplessHashMap<K, HeaplessVec<V, M>, N> = HeaplessHashMap::new();

    for (key, val) in iter {
        if let Some(vec) = lookup.get_mut(&key) {
            vec.push(val).map_err(GroupMapError::VecInsertError)?;
        } else {
            let mut vec = HeaplessVec::new();
            vec.push(val).map_err(GroupMapError::VecInsertError)?;
            lookup
                .insert(key, vec)
                .map_err(|(k, _)| GroupMapError::HashMapInsertError(k))?;
        }
    }

    Ok(lookup)
}
