use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// Observer trait.
pub trait Observe<T> {
    /// Updates this observer with an item.
    fn update(&mut self, item: T);
}

impl<T> Observe<T> for Vec<T> {
    fn update(&mut self, item: T) {
        self.push(item);
    }
}
impl<T> Observe<T> for VecDeque<T> {
    fn update(&mut self, item: T) {
        self.push_back(item);
    }
}
impl<K: Eq + Hash, V> Observe<(K, V)> for HashMap<K, V> {
    fn update(&mut self, item: (K, V)) {
        let (k, v) = item;
        self.insert(k, v);
    }
}
impl<T: Eq + Hash> Observe<T> for HashSet<T> {
    fn update(&mut self, item: T) {
        self.insert(item);
    }
}
impl<K: Eq + Ord, V> Observe<(K, V)> for BTreeMap<K, V> {
    fn update(&mut self, item: (K, V)) {
        let (k, v) = item;
        self.insert(k, v);
    }
}
impl<T: Eq + Ord> Observe<T> for BTreeSet<T> {
    fn update(&mut self, item: T) {
        self.insert(item);
    }
}
impl Observe<char> for String {
    fn update(&mut self, item: char) {
        self.push(item);
    }
}
impl Observe<&str> for String {
    fn update(&mut self, item: &str) {
        self.push_str(item);
    }
}
