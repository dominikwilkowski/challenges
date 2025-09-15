use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Node<V> {
	index: usize,
	value: V,
	pub next: Option<usize>,
	pub prev: Option<usize>,
}

impl<V> Node<V> {
	fn new(index: usize, value: V, next: Option<usize>, prev: Option<usize>) -> Self {
		Self {
			index,
			value,
			next,
			prev,
		}
	}
}

#[derive(Debug)]
pub struct LruCache<K, V>
where
	K: Clone + Eq + std::hash::Hash,
{
	items: Vec<Option<Node<V>>>,
	map: HashMap<usize, usize>,
	lookup: HashMap<K, usize>,
	head: Option<usize>,
	tail: Option<usize>,
	len: usize,
	index: usize,
}

// items:
// Write A
// map: A:0
// items: A{prev:None, next:None}
// Write B
// map: A:0 B:1
// items: A{prev:None, next:1} B{prev:0, next:None}
// Write C
// map: A:0 B:1 C:2
// items: A{prev:None, next:1} B{prev:0, next:2} C{prev:1, next:None}
// ...
// items: A{prev:None, next:1} B{prev:0, next:2} C{prev:1, next:3} D{prev:2, next:None}
// A B C D
// map: A:0, B:1, C:2, D:3
// Write E
// items: E{prev:3, next:None} B{prev:None, next:2} C{prev:1, next:3} D{prev:2, next:0}
// B C D E
// map: B:1, C:2, D:3, E:0
// Read B
// items: E{prev:3, next:1} B{prev:0, next:None} C{prev:None, next:3} D{prev:2, next:0}
// C D E B
// map: B:1, C:2, D:3, E:0

impl<K, V> LruCache<K, V>
where
	K: Clone + Eq + std::hash::Hash,
{
	pub fn new(size: usize) -> Self {
		if size == 0 {
			panic!("Capacity must be greater than 0");
		}

		Self {
			items: Vec::with_capacity(size),
			map: HashMap::with_capacity(size),
			lookup: HashMap::with_capacity(size),
			head: None,
			tail: None,
			len: 0,
			index: 0,
		}
	}

	pub fn write(&mut self, key: K, value: V) {
		let tail = self.tail;
		// TODO:
		// - fix for capacity: 1
		// - fix if keys already exist
		// - wrap key in Arc to avoid clones
		// - add helpers
		if self.len == self.items.capacity() {
			let head_node = self.head.unwrap();
			let new_head_node = self.items[head_node].as_ref().unwrap().next.unwrap();
			let key_to_remove = &self.items[head_node].as_ref().unwrap().key;
			self.map.remove(key_to_remove);
			self.items[head_node] = Some(Node::new(key.clone(), value, None, tail));
			self.map.insert(key, head_node);
			self.tail = Some(head_node);
			self.head = Some(new_head_node);
			self.items[new_head_node].as_mut().unwrap().prev = None;
			self.items[tail.unwrap()].as_mut().unwrap().next = Some(head_node);
		} else {
			self.lookup.insert(key, self.index);
			self.items.push(Some(Node::new(self.index, value, None, tail)));
			self.tail = Some(self.items.len() - 1);
			if self.items.len() == 1 {
				self.head = Some(self.items.len() - 1);
			}
			self.map.insert(self.index, self.items.len() - 1);
			self.len += 1;
			if let Some(tail_node) = tail {
				self.items[tail_node].as_mut().unwrap().next = Some(self.items.len() - 1);
			}

			self.index += 1;
		}
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
	fn write_item_test() {
		let mut cache = LruCache::new(3);

		cache.write(1, "one");
		assert_eq!(cache.items, vec![Some(Node::new(1, "one", None, None))]);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(2, "two");
		assert_eq!(
			cache.items,
			vec![
				Some(Node::new(1, "one", Some(1), None)),
				Some(Node::new(2, "two", None, Some(0))),
			]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.map.get(&2), Some(&1));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(1));
		assert_eq!(cache.len, 2);

		cache.write(3, "three");
		assert_eq!(
			cache.items,
			vec![
				Some(Node::new(1, "one", Some(1), None)),
				Some(Node::new(2, "two", Some(2), Some(0))),
				Some(Node::new(3, "three", None, Some(1))),
			]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.map.get(&2), Some(&1));
		assert_eq!(cache.map.get(&3), Some(&2));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(2));
		assert_eq!(cache.len, 3);

		cache.write(4, "four");
		assert_eq!(
			cache.items,
			vec![
				Some(Node::new(4, "four", None, Some(2))),
				Some(Node::new(2, "two", Some(2), None)),
				Some(Node::new(3, "three", Some(0), Some(1))),
			]
		);
		println!("{:?}", cache.map);
		assert_eq!(cache.map.get(&1), None);
		assert_eq!(cache.map.get(&2), Some(&1));
		assert_eq!(cache.map.get(&3), Some(&2));
		assert_eq!(cache.map.get(&4), Some(&0));
		assert_eq!(cache.head, Some(1));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 3);
	}

	// #[test]
	// fn lru_cache_test() {
	// 	let mut cache = LruCache::new(2);
	// 	cache.write(1, "one");
	// 	cache.write(2, "two");
	// 	assert_eq!(cache.read(&1), Some(&"one"));
	// 	assert_eq!(cache.read(&2), Some(&"two"));
	// 	cache.write(3, "three");
	// 	assert_eq!(cache.read(&1), None);
	// 	assert_eq!(cache.read(&2), Some(&"two"));
	// 	assert_eq!(cache.read(&3), Some(&"three"));
	// 	cache.write(4, "four");
	// 	assert_eq!(cache.read(&2), None);
	// 	assert_eq!(cache.read(&3), Some(&"three"));
	// 	assert_eq!(cache.read(&4), Some(&"four"));
	// }
}
