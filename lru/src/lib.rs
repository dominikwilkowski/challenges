use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Node<K, V> {
	key: K,
	pub value: V,
	pub prev: Option<usize>,
	pub next: Option<usize>,
}

impl<K, V> Node<K, V> {
	fn new(key: K, value: V, prev: Option<usize>, next: Option<usize>) -> Self {
		Self { key, value, prev, next }
	}
}

#[derive(Debug)]
pub struct LruCache<K, V>
where
	K: Clone + Eq + std::hash::Hash,
{
	items: Vec<Option<Node<K, V>>>,
	map: HashMap<K, usize>,
	head: Option<usize>,
	tail: Option<usize>,
	len: usize,
	capacity: usize,
}

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
			head: None,
			tail: None,
			len: 0,
			capacity: size,
		}
	}

	fn move_to_tail(&mut self, index: usize) {
		// moving [index] to tail -->
		// head .. - [index_prev] - [index] - [index_next] - .. tail
		//                      ------^
		if self.tail == Some(index) {
			// [index] already at tail
			// .. - [index_prev] - [index]
			return;
		}

		let index_node = self.items[index].as_ref().expect("BUG: node index not found");

		// get the item after the item to be moved [index] (and we know there is an item because we are not at the tail)
		// .. - [index_prev] - [index] - [index_next] - ..
		//                               ------^
		let index_next = index_node.next.expect("BUG: item not at tail did not have a next link");

		// make prev item of this node point to next item
		// .. - [index_prev] -- [index_next] - ..
		if let Some(index_prev) = index_node.prev {
			// node was inside the chain
			// .. - [index_prev] - [index] - [index_next] - ..

			// so last item that came after our new tail node now points to the previous prev item
			// .. - [index_prev] <- [index_next] - ..
			self.items[index_next].as_mut().expect("BUG: node index_next not found").prev = Some(index_prev);

			// and the item that came before our new tail now points to the previous next item
			// .. - [index_prev] -> [index_next] - ..
			self.items[index_prev].as_mut().expect("BUG: node index_prev not found").next = Some(index_next);
		} else {
			// new tail node was at the head
			// [index] - [index_next] - ..

			// so last item that came after our new tail node is now head and prev is None
			self.items[index_next].as_mut().expect("BUG: node index_next not found").prev = None;

			// the item after our new tail node will now be head
			self.head = Some(index_next);
		}

		// point index prev to old tail
		// head .. - [index_prev] - [index_next] - [old_tail] <- [index]
		self.items[index].as_mut().expect("BUG: node index not found").prev = self.tail;

		// point old tail node to index
		// head .. - [index_prev] - [index_next] - [old_tail] -> [index]
		// Note: the unwrap is fine due to the early return check at the start of this function
		self.items[self.tail.unwrap()].as_mut().expect("BUG: tail node not found").next = Some(index);

		// point tail to this node
		self.tail = Some(index);

		// make new tail next node none
		// head .. - [index_prev] - [index_next] - [index] -> None
		self.items[index].as_mut().expect("BUG: node index not found").next = None;
	}

	pub fn write(&mut self, key: K, value: V) {
		let tail = self.tail;
		// TODO:
		// - fix unwraps
		// - add helper functions

		// UPDATE PATH
		if let Some(new_tail) = self.map.get(&key) {
			// update value
			self.items[*new_tail].as_mut().unwrap().value = value;

			self.move_to_tail(*new_tail);
		// EVICTION PATH
		} else if self.len == self.capacity {
			// get previous head node
			let head_node = self.head.unwrap();

			// cache key of previous head node
			let key_to_remove = &self.items[head_node].as_ref().unwrap().key;

			// cache node after previous head node
			let new_head_node = self.items[head_node].as_ref().unwrap().next;

			// remove old mapping
			self.map.remove(key_to_remove);

			// overwrite new node to where the last head node was
			// Note: We clone here assuming that if a key is used that is expensive to clone they would use Arc to make it cheaper
			self.items[head_node] = Some(Node::new(key.clone(), value, if self.capacity == 1 { None } else { tail }, None));

			// add new mapping
			self.map.insert(key, head_node);

			if let Some(new_head_node) = new_head_node {
				// point head to new node and tail to node after old head node
				self.tail = Some(head_node);
				self.head = Some(new_head_node);

				// connect node after old head to None as it is now new head node
				self.items[new_head_node].as_mut().unwrap().prev = None;

				// connect old tail to new tail node
				self.items[tail.unwrap()].as_mut().unwrap().next = Some(head_node);
			}
		// INSERTION PATH
		} else {
			// add new node to items with key
			// Note: We clone here assuming that if a key is used that is expensive to clone they would use Arc to make it cheaper
			self.items.push(Some(Node::new(key.clone(), value, tail, None)));

			// point tail to new node
			self.tail = Some(self.items.len() - 1);

			// if first node, also point head to new node
			if self.items.len() == 1 {
				self.head = Some(self.items.len() - 1);
			}

			// record new nodes index into map
			self.map.insert(key, self.items.len() - 1);

			// increment length
			self.len += 1;

			// point previous tail node to new tail to complete the chain
			if let Some(tail_node) = tail {
				self.items[tail_node].as_mut().unwrap().next = Some(self.items.len() - 1);
			}
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

	pub fn is_empty(&self) -> bool {
		self.len == 0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn write_items_test() {
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
				Some(Node::new(1, "one", None, Some(1))),
				Some(Node::new(2, "two", Some(0), None)),
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
				Some(Node::new(1, "one", None, Some(1))),
				Some(Node::new(2, "two", Some(0), Some(2))),
				Some(Node::new(3, "three", Some(1), None)),
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
				Some(Node::new(4, "four", Some(2), None)),
				Some(Node::new(2, "two", None, Some(2))),
				Some(Node::new(3, "three", Some(1), Some(0))),
			]
		);
		assert_eq!(cache.map.get(&1), None);
		assert_eq!(cache.map.get(&2), Some(&1));
		assert_eq!(cache.map.get(&3), Some(&2));
		assert_eq!(cache.map.get(&4), Some(&0));
		assert_eq!(cache.head, Some(1));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 3);
	}

	#[test]
	fn write_single_capacity_test() {
		let mut cache = LruCache::new(1);

		cache.write(1, "one");
		assert_eq!(cache.items, vec![Some(Node::new(1, "one", None, None))]);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(2, "two");
		assert_eq!(cache.items, vec![Some(Node::new(2, "two", None, None)),]);
		assert_eq!(cache.map.get(&1), None);
		assert_eq!(cache.map.get(&2), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(3, "three");
		assert_eq!(cache.items, vec![Some(Node::new(3, "three", None, None)),]);
		assert_eq!(cache.map.get(&1), None);
		assert_eq!(cache.map.get(&2), None);
		assert_eq!(cache.map.get(&3), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);
	}

	#[test]
	fn write_existing_item_filling_test() {
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
				Some(Node::new(1, "one", None, Some(1))),
				Some(Node::new(2, "two", Some(0), None)),
			]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.map.get(&2), Some(&1));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(1));
		assert_eq!(cache.len, 2);

		cache.write(1, "three");
		assert_eq!(
			cache.items,
			vec![
				Some(Node::new(1, "three", Some(1), None)),
				Some(Node::new(2, "two", None, Some(0))),
			]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.map.get(&2), Some(&1));
		assert_eq!(cache.head, Some(1));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 2);
	}

	#[test]
	fn write_existing_item_full_test() {
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
				Some(Node::new(1, "one", None, Some(1))),
				Some(Node::new(2, "two", Some(0), None)),
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
				Some(Node::new(1, "one", None, Some(1))),
				Some(Node::new(2, "two", Some(0), Some(2))),
				Some(Node::new(3, "three", Some(1), None)),
			]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.map.get(&2), Some(&1));
		assert_eq!(cache.map.get(&3), Some(&2));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(2));
		assert_eq!(cache.len, 3);

		cache.write(2, "four");
		assert_eq!(
			cache.items,
			vec![
				Some(Node::new(1, "one", None, Some(2))),
				Some(Node::new(2, "four", Some(2), None)),
				Some(Node::new(3, "three", Some(0), Some(1))),
			]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.map.get(&2), Some(&1));
		assert_eq!(cache.map.get(&3), Some(&2));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(1));
		assert_eq!(cache.len, 3);
	}

	#[test]
	fn write_existing_item_capacity_one_test() {
		let mut cache = LruCache::new(1);

		cache.write(1, "one");
		assert_eq!(cache.items, vec![Some(Node::new(1, "one", None, None))]);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(2, "two");
		assert_eq!(cache.items, vec![Some(Node::new(2, "two", None, None))]);
		assert_eq!(cache.map.get(&1), None);
		assert_eq!(cache.map.get(&2), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(3, "three");
		assert_eq!(cache.items, vec![Some(Node::new(3, "three", None, None))]);
		assert_eq!(cache.map.get(&1), None);
		assert_eq!(cache.map.get(&2), None);
		assert_eq!(cache.map.get(&3), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);
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
