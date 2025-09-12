use std::collections::HashMap;

struct Node<K, V> {
	key: K,
	value: V,
	next: Option<usize>,
	prev: Option<usize>,
}

impl<K, V> Node<K, V> {
	fn new(key: K, value: V, next: Option<usize>, prev: Option<usize>) -> Self {
		Self { key, value, next, prev }
	}
}

pub struct LruCache<K, V> {
	items: Vec<Option<Node<K, V>>>,
	map: HashMap<K, usize>,
	head: Option<usize>,
	tail: Option<usize>,
}

impl<K, V> LruCache<K, V> {
	pub fn new(size: usize) -> Self {
		Self {
			items: Vec::with_capacity(size),
			map: HashMap::with_capacity(size),
			head: None,
			tail: None,
		}
	}

	pub fn add(&mut self, key: K, value: V) {
		// push into self.items
		todo!()
	}

	pub fn get(&mut self, key: &K) -> Option<&V> {
		todo!()
	}

	pub fn remove(&mut self, key: &K) -> Result<(), ()> {
		todo!()
	}

	pub fn clear(&mut self) {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_lru_cache() {
		let mut cache = LruCache::new(2);
		cache.add(1, "one");
		cache.add(2, "two");
		assert_eq!(cache.get(&1), Some(&"one"));
		assert_eq!(cache.get(&2), Some(&"two"));
		cache.add(3, "three");
		assert_eq!(cache.get(&1), None);
		assert_eq!(cache.get(&2), Some(&"two"));
		assert_eq!(cache.get(&3), Some(&"three"));
		cache.add(4, "four");
		assert_eq!(cache.get(&2), None);
		assert_eq!(cache.get(&3), Some(&"three"));
		assert_eq!(cache.get(&4), Some(&"four"));
	}
}
