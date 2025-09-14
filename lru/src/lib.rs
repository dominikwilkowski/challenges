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
	len: usize,
}

// items: A{prev:None, next:1} B{prev:0, next:2} C{prev:1, next:3} D{prev:2, next:None}
// A B C D
// map: A:0, B:1, C:2, D:3
// Write E
// items: E{prev:3, next:None} B{prev:None, next:2} C{prev:1, next:3} D{prev:2, next:0}
// B C D E
// map: E:0, B:1, C:2, D:3
// Read B
// items: E{prev:3, next:1} B{prev:0, next:None} C{prev:None, next:3} D{prev:2, next:0}
// C D E B

impl<K, V> LruCache<K, V> {
	pub fn new(size: usize) -> Self {
		Self {
			items: Vec::with_capacity(size),
			map: HashMap::with_capacity(size),
			head: None,
			tail: None,
			len: 0,
		}
	}

	pub fn write(&mut self, key: K, value: V) {
		// push into self.items
		// get index from self.items
		// add index and key to self.map with self.tail as prev and self.head as next
		// add to tail
		// if fist item, add to head
		todo!()
	}

	pub fn read(&mut self, key: &K) -> Option<&V> {
		// get index from self.map via key
		// remove item from map
		// add item back to map
		// return value from self.items
		todo!()
	}

	pub fn delete(&mut self, key: &K) -> Result<(), ()> {
		todo!()
	}

	pub fn clear(&mut self) {
		todo!()
	}

	pub fn len(&self) -> usize {
		self.len
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn lru_cache_test() {
		let mut cache = LruCache::new(2);
		cache.write(1, "one");
		cache.write(2, "two");
		assert_eq!(cache.read(&1), Some(&"one"));
		assert_eq!(cache.read(&2), Some(&"two"));
		cache.write(3, "three");
		assert_eq!(cache.read(&1), None);
		assert_eq!(cache.read(&2), Some(&"two"));
		assert_eq!(cache.read(&3), Some(&"three"));
		cache.write(4, "four");
		assert_eq!(cache.read(&2), None);
		assert_eq!(cache.read(&3), Some(&"three"));
		assert_eq!(cache.read(&4), Some(&"four"));
	}
}
