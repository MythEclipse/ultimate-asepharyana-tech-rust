use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

/// Convert Vec<T> to Vec<U>.
pub fn map_vec<T, U, F>(vec: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    vec.into_iter().map(f).collect()
}

/// Convert &[T] to Vec<U>.
pub fn map_slice<T, U, F>(slice: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    slice.iter().map(f).collect()
}

/// Convert Vec<String> to Vec<&str>.
pub fn strings_to_strs(strings: &[String]) -> Vec<&str> {
    strings.iter().map(|s| s.as_str()).collect()
}

/// Convert Vec<&str> to Vec<String>.
pub fn strs_to_strings(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

/// Convert slice to fixed array.
pub fn slice_to_array<T: Copy, const N: usize>(slice: &[T]) -> Option<[T; N]> {
    slice.try_into().ok()
}

/// Convert Vec to VecDeque.
pub fn vec_to_deque<T>(v: Vec<T>) -> VecDeque<T> {
    VecDeque::from(v)
}

/// Convert VecDeque to Vec.
pub fn deque_to_vec<T>(d: VecDeque<T>) -> Vec<T> {
    d.into_iter().collect()
}

/// Convert Vec to LinkedList.
pub fn vec_to_linked_list<T>(v: Vec<T>) -> LinkedList<T> {
    v.into_iter().collect()
}

/// Convert LinkedList to Vec.
pub fn linked_list_to_vec<T>(l: LinkedList<T>) -> Vec<T> {
    l.into_iter().collect()
}

/// Convert Vec to HashSet.
pub fn vec_to_hashset<T: Eq + Hash>(v: Vec<T>) -> HashSet<T> {
    v.into_iter().collect()
}

/// Convert HashSet to Vec.
pub fn hashset_to_vec<T>(s: HashSet<T>) -> Vec<T> {
    s.into_iter().collect()
}

/// Convert Vec to BTreeSet.
pub fn vec_to_btreeset<T: Ord>(v: Vec<T>) -> BTreeSet<T> {
    v.into_iter().collect()
}

/// Convert BTreeSet to Vec.
pub fn btreeset_to_vec<T>(s: BTreeSet<T>) -> Vec<T> {
    s.into_iter().collect()
}

/// Convert Vec of tuples to HashMap.
pub fn vec_to_hashmap<K: Eq + Hash, V>(v: Vec<(K, V)>) -> HashMap<K, V> {
    v.into_iter().collect()
}

/// Convert HashMap to Vec of tuples.
pub fn hashmap_to_vec<K, V>(m: HashMap<K, V>) -> Vec<(K, V)> {
    m.into_iter().collect()
}

/// Convert Vec of tuples to BTreeMap.
pub fn vec_to_btreemap<K: Ord, V>(v: Vec<(K, V)>) -> BTreeMap<K, V> {
    v.into_iter().collect()
}

/// Convert BTreeMap to Vec of tuples.
pub fn btreemap_to_vec<K, V>(m: BTreeMap<K, V>) -> Vec<(K, V)> {
    m.into_iter().collect()
}

/// Convert HashMap to BTreeMap.
pub fn hashmap_to_btreemap<K: Ord + Hash, V>(m: HashMap<K, V>) -> BTreeMap<K, V> {
    m.into_iter().collect()
}

/// Convert BTreeMap to HashMap.
pub fn btreemap_to_hashmap<K: Eq + Hash, V>(m: BTreeMap<K, V>) -> HashMap<K, V> {
    m.into_iter().collect()
}
