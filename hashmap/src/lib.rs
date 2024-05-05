#![allow(unused)]
use core::hash;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    mem,
};

const INITIAL_SIZE: usize = 1;

pub struct Hashmap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K, V> Hashmap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Hashmap {
            buckets: Vec::new(),
            size: 0,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.size >= 3 * self.buckets.len() {
            self.resize();
        }
        let idx = self.bucket_of(&key);
        let bucket = &mut self.buckets[idx];
        for &mut (ref k, ref mut v) in bucket.iter_mut() {
            if *k == key {
                use std::mem;
                return Some(mem::replace(v, value));
            }
        }
        bucket.push((key, value));
        self.size += 1;
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let idx = self.bucket_of(&key);
        let bucket = &self.buckets[idx];
        bucket.iter().find(|(k, v)| k == key).map(|(_, v)| v)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let idx = self.bucket_of(&key);
        let bucket = &self.buckets[idx];
        bucket.iter().find(|(k, _)| k == key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let idx = self.bucket_of(&key);
        let bucket = &mut self.buckets[idx];
        let i = bucket.iter().position(|(k, _)| k == key)?;
        self.size -= 1;
        Some(bucket.swap_remove(i).1)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn resize(&mut self) {
        let new_size = match self.buckets.len() {
            0 => INITIAL_SIZE,
            n => 2 * n,
        };
        let mut new_buckets = Vec::with_capacity(new_size);
        new_buckets.extend((0..new_size).map(|_| Vec::new()));
        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let idx = (hasher.finish() % (new_size as u64)) as usize;
            new_buckets[idx].push((key, value));
        }
        let _ = mem::replace(&mut self.buckets, new_buckets);
    }

    fn bucket_of(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % (self.buckets.len() as u64)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = Hashmap::new();
        map.insert("tea", 100);
        assert_eq!(map.get(&"tea"), Some(&100));
        assert_eq!(map.contains_key(&"tea"), true);
        assert_eq!(map.len(), 1);
        assert_eq!(map.remove(&"tea"), Some(100));
        assert_eq!(map.len(), 0);
        assert_eq!(map.get(&"tea"), None);
    }
}
