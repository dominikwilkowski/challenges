use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Node<K, V> {
	key: K,
	pub value: V,
	pub prev: Option<usize>,
	pub next: Option<usize>,
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
		let index_next = index_node.next.expect("BUG: non-tail-item did not have a next link");

		// detach index from chain
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

		{
			let index_node = self.items[index].as_mut().expect("BUG: node index not found");

			// point index prev to old tail
			// head .. - [index_prev] - [index_next] - [old_tail] <- [index]
			index_node.prev = self.tail;

			// make new tail next node none
			// head .. - [index_prev] - [index_next] - [index] -> None
			index_node.next = None;
		}

		// head .. - [index_prev] - [index_next] - [old_tail] -> [index]
		// Note: the unwrap is fine due to the early return check at the start of this function
		self.items[self.tail.unwrap()].as_mut().expect("BUG: tail node not found").next = Some(index);

		// point tail to this node
		self.tail = Some(index);
	}

	pub fn write(&mut self, key: K, value: V) {
		let tail = self.tail;
		// TODO:
		// - add helper functions

		// UPDATE PATH
		if let Some(new_tail) = self.map.get(&key) {
			// update value
			self.items[*new_tail].as_mut().expect("BUG: node from map not found").value = value;

			self.move_to_tail(*new_tail);
		// EVICTION PATH
		} else if self.len == self.capacity {
			// get previous head node
			let head_index = self.head.expect("BUG: no head node was set");
			let head_node = self.items[head_index].as_ref().expect("BUG: head node not found");

			// cache key of previous head node
			let key_to_remove = &head_node.key;

			// cache node after previous head node
			let new_head_node = head_node.next;

			// remove old mapping
			self.map.remove(key_to_remove);

			// overwrite new node to where the last head node was
			// Note: We clone here assuming that if a key is used that is expensive to clone they would use Arc to make it cheaper
			self.items[head_index] = Some(Node {
				key: key.clone(),
				value,
				prev: if self.capacity == 1 { None } else { tail },
				next: None,
			});

			// add new mapping
			self.map.insert(key, head_index);

			if let Some(new_head_node) = new_head_node {
				// point head to new node and tail to node after old head node
				self.tail = Some(head_index);
				self.head = Some(new_head_node);

				// connect node after old head to None as it is now new head node
				self.items[new_head_node].as_mut().expect("BUG: new head node not found").prev = None;

				// connect old tail to new tail node
				self.items[tail.expect("BUG: tail node not set")].as_mut().expect("BUG: tail node not found").next =
					Some(head_index);
			}
		// INSERTION PATH
		} else {
			// add new node to items with key
			// Note: We clone here assuming that if a key is used that is expensive to clone they would use Arc to make it cheaper
			self.items.push(Some(Node {
				key: key.clone(),
				value,
				prev: tail,
				next: None,
			}));

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
				self.items[tail_node].as_mut().expect("BUG: tail node not found").next = Some(self.items.len() - 1);
			}
		}
	}

	pub fn read(&mut self, key: &K) -> Option<&V> {
		let index = match self.map.get(key).copied() {
			Some(idx) => idx,
			None => return None,
		};

		self.move_to_tail(index);
		Some(&self.items[index].as_ref().expect("BUG: node not found").value)
	}

	pub fn delete(&mut self, key: &K) -> Result<(), ()> {
		let index = match self.map.get(key).copied() {
			Some(idx) => idx,
			None => return Err(()),
		};

		let (prev, next) = {
			let item = self.items[index].as_ref().expect("BUG: node not found");
			(item.prev, item.next)
		};

		// attach prev to next
		if let Some(prev) = prev {
			self.items[prev].as_mut().expect("BUG: prev node not found").next = next;
		}

		// attach next to prev
		if let Some(next) = next {
			self.items[next].as_mut().expect("BUG: next node not found").prev = prev;
		}

		// remove from map
		self.map.remove(&key);

		// remove item
		self.items[index] = None;

		// if the removed node was at the head replace it with the item after
		if self.head == Some(index) {
			self.head = next;
		}

		// if the removed node was at the tail replace it with the item before
		if self.tail == Some(index) {
			self.tail = prev;
		}

		self.len -= 1;

		Ok(())
	}

	pub fn clear(&mut self) {
		self.items.clear();
		self.head = None;
		self.tail = None;
		self.map.clear();
		self.len = 0;
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
	use std::{cell::RefCell, rc::Rc};

	#[derive(Debug, Clone, PartialEq)]
	struct DropSpy {
		id: usize,
		log: Rc<RefCell<Vec<usize>>>,
	}

	impl Drop for DropSpy {
		fn drop(&mut self) {
			self.log.borrow_mut().push(self.id);
		}
	}

	#[should_panic]
	#[test]
	fn zero_capacity_test() {
		LruCache::<i32, &str>::new(0);
	}

	#[test]
	fn write_items_test() {
		let mut cache = LruCache::new(3);

		cache.write(1, "one");
		assert_eq!(
			cache.items,
			vec![Some(Node {
				key: 1,
				value: "one",
				prev: None,
				next: None
			})]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(2, "two");
		assert_eq!(
			cache.items,
			vec![
				Some(Node {
					key: 1,
					value: "one",
					prev: None,
					next: Some(1)
				}),
				Some(Node {
					key: 2,
					value: "two",
					prev: Some(0),
					next: None
				}),
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
				Some(Node {
					key: 1,
					value: "one",
					prev: None,
					next: Some(1)
				}),
				Some(Node {
					key: 2,
					value: "two",
					prev: Some(0),
					next: Some(2)
				}),
				Some(Node {
					key: 3,
					value: "three",
					prev: Some(1),
					next: None
				}),
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
				Some(Node {
					key: 4,
					value: "four",
					prev: Some(2),
					next: None
				}),
				Some(Node {
					key: 2,
					value: "two",
					prev: None,
					next: Some(2)
				}),
				Some(Node {
					key: 3,
					value: "three",
					prev: Some(1),
					next: Some(0)
				}),
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
		assert_eq!(
			cache.items,
			vec![Some(Node {
				key: 1,
				value: "one",
				next: None,
				prev: None
			})]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(2, "two");
		assert_eq!(
			cache.items,
			vec![Some(Node {
				key: 2,
				value: "two",
				next: None,
				prev: None
			}),]
		);
		assert_eq!(cache.map.get(&1), None);
		assert_eq!(cache.map.get(&2), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(3, "three");
		assert_eq!(
			cache.items,
			vec![Some(Node {
				key: 3,
				value: "three",
				next: None,
				prev: None
			}),]
		);
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
		assert_eq!(
			cache.items,
			vec![Some(Node {
				key: 1,
				value: "one",
				prev: None,
				next: None
			})]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(2, "two");
		assert_eq!(
			cache.items,
			vec![
				Some(Node {
					key: 1,
					value: "one",
					prev: None,
					next: Some(1)
				}),
				Some(Node {
					key: 2,
					value: "two",
					prev: Some(0),
					next: None
				}),
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
				Some(Node {
					key: 1,
					value: "three",
					prev: Some(1),
					next: None
				}),
				Some(Node {
					key: 2,
					value: "two",
					prev: None,
					next: Some(0)
				}),
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
		assert_eq!(
			cache.items,
			vec![Some(Node {
				key: 1,
				value: "one",
				prev: None,
				next: None
			})]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(2, "two");
		assert_eq!(
			cache.items,
			vec![
				Some(Node {
					key: 1,
					value: "one",
					prev: None,
					next: Some(1)
				}),
				Some(Node {
					key: 2,
					value: "two",
					prev: Some(0),
					next: None
				}),
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
				Some(Node {
					key: 1,
					value: "one",
					prev: None,
					next: Some(1)
				}),
				Some(Node {
					key: 2,
					value: "two",
					prev: Some(0),
					next: Some(2)
				}),
				Some(Node {
					key: 3,
					value: "three",
					prev: Some(1),
					next: None
				}),
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
				Some(Node {
					key: 1,
					value: "one",
					prev: None,
					next: Some(2)
				}),
				Some(Node {
					key: 2,
					value: "four",
					prev: Some(2),
					next: None
				}),
				Some(Node {
					key: 3,
					value: "three",
					prev: Some(0),
					next: Some(1)
				}),
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
		assert_eq!(
			cache.items,
			vec![Some(Node {
				key: 1,
				value: "one",
				prev: None,
				next: None
			})]
		);
		assert_eq!(cache.map.get(&1), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(2, "two");
		assert_eq!(
			cache.items,
			vec![Some(Node {
				key: 2,
				value: "two",
				prev: None,
				next: None
			})]
		);
		assert_eq!(cache.map.get(&1), None);
		assert_eq!(cache.map.get(&2), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);

		cache.write(3, "three");
		assert_eq!(
			cache.items,
			vec![Some(Node {
				key: 3,
				value: "three",
				prev: None,
				next: None
			})]
		);
		assert_eq!(cache.map.get(&1), None);
		assert_eq!(cache.map.get(&2), None);
		assert_eq!(cache.map.get(&3), Some(&0));
		assert_eq!(cache.head, Some(0));
		assert_eq!(cache.tail, Some(0));
		assert_eq!(cache.len, 1);
	}

	#[test]
	fn read_test() {
		let mut cache = LruCache::new(2);
		cache.write(1, "one");
		cache.write(2, "two");
		assert_eq!(cache.map.len(), 2);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.read(&1), Some(&"one"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.read(&2), Some(&"two"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.read(&2), Some(&"two"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "two");

		cache.write(3, "three");
		assert_eq!(cache.map.len(), 2);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "three");
		assert_eq!(cache.read(&1), None);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "three");
		assert_eq!(cache.read(&2), Some(&"two"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "three");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.read(&2), Some(&"two"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "three");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.read(&3), Some(&"three"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "three");

		cache.write(4, "four");
		assert_eq!(cache.map.len(), 2);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "three");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "four");
		assert_eq!(cache.read(&2), None);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "three");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "four");
		assert_eq!(cache.read(&3), Some(&"three"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "four");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "three");
		assert_eq!(cache.read(&4), Some(&"four"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "three");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "four");
	}

	#[test]
	fn clear_test() {
		let log = Rc::new(RefCell::new(Vec::new()));
		let mut cache = LruCache::new(2);

		cache.write(
			1,
			DropSpy {
				id: 1,
				log: log.clone(),
			},
		);
		cache.write(
			2,
			DropSpy {
				id: 2,
				log: log.clone(),
			},
		);
		cache.clear();

		{
			let mut seen = log.borrow().clone();
			seen.sort();
			assert_eq!(seen, vec![1, 2]);
		}

		assert_eq!(cache.len(), 0);
		assert_eq!(cache.capacity, 2);
		assert!(cache.items.is_empty());
		assert_eq!(cache.head, None);
		assert_eq!(cache.tail, None);
		assert!(cache.map.is_empty());

		let item = DropSpy {
			id: 3,
			log: log.clone(),
		};
		cache.write(3, item.clone());
		assert_eq!(cache.len(), 1);
		assert_eq!(cache.capacity, 2);
		assert_eq!(cache.read(&1), None);
		assert_eq!(cache.read(&2), None);
		assert_eq!(cache.read(&3), Some(&item));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, item);
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, item);
		assert_eq!(cache.map.len(), 1);

		{
			let mut seen = log.borrow().clone();
			seen.sort();
			assert_eq!(seen, vec![1, 2]);
		}
	}

	#[test]
	fn delete_missing_test() {
		let mut cache = LruCache::<i32, &str>::new(2);
		assert_eq!(cache.delete(&42), Err(()));
		assert_eq!(cache.len(), 0);
		assert!(cache.map.is_empty());
		assert_eq!(cache.head, None);
		assert_eq!(cache.tail, None);

		let mut cache = LruCache::new(2);
		cache.write(1, "one");
		cache.write(2, "two");
		let head = cache.head;
		let tail = cache.tail;

		assert_eq!(cache.delete(&999), Err(()));
		assert_eq!(cache.len(), 2);
		assert_eq!(cache.map.len(), 2);
		assert_eq!(cache.head, head);
		assert_eq!(cache.tail, tail);
	}

	#[test]
	fn delete_single_test() {
		let mut cache = LruCache::new(2);
		cache.write(1, "one");
		assert_eq!(cache.len(), 1);
		assert_eq!(cache.delete(&1), Ok(()));
		assert_eq!(cache.len(), 0);
		assert!(cache.map.is_empty());
		assert_eq!(cache.head, None);
		assert_eq!(cache.tail, None);
	}

	#[test]
	fn delete_head_test() {
		let mut cache = LruCache::new(2);
		cache.write(1, "one");
		cache.write(2, "two");
		assert_eq!(cache.map.len(), 2);
		assert_eq!(cache.len(), 2);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "two");

		assert_eq!(cache.delete(&1), Ok(()));
		assert_eq!(cache.map.len(), 1);
		assert_eq!(cache.len(), 1);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "two");
	}

	#[test]
	fn delete_tail_test() {
		let mut cache = LruCache::new(2);
		cache.write(1, "one");
		cache.write(2, "two");
		assert_eq!(cache.map.len(), 2);
		assert_eq!(cache.len(), 2);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "two");

		assert_eq!(cache.delete(&2), Ok(()));
		assert_eq!(cache.map.len(), 1);
		assert_eq!(cache.len(), 1);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "one");
	}

	#[test]
	fn delete_center_test() {
		let mut cache = LruCache::new(3);
		cache.write(1, "one");
		cache.write(2, "two");
		cache.write(3, "three");
		assert_eq!(cache.map.len(), 3);
		assert_eq!(cache.len(), 3);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "three");

		assert_eq!(cache.delete(&2), Ok(()));
		assert_eq!(cache.map.len(), 2);
		assert_eq!(cache.len(), 2);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "three");
	}

	#[test]
	fn complex_crud_flow_test() {
		let mut cache = LruCache::new(3);
		assert_eq!(cache.len(), 0);
		assert!(cache.is_empty());
		assert!(cache.map.is_empty());
		assert_eq!(cache.head, None);
		assert_eq!(cache.tail, None);

		// write up to capacity
		cache.write(1, "one");
		cache.write(2, "two");
		cache.write(3, "three");
		assert_eq!(cache.len(), 3);
		assert_eq!(cache.map.len(), 3);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "three");

		// read middle
		assert_eq!(cache.read(&2), Some(&"two"));
		assert_eq!(cache.len(), 3);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "two");

		// read head
		assert_eq!(cache.read(&1), Some(&"one"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "three");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "one");

		// delete missing is a no-op
		assert_eq!(cache.delete(&999), Err(()));
		assert_eq!(cache.len(), 3);
		assert_eq!(cache.map.len(), 3);

		// delete current head
		assert_eq!(cache.delete(&3), Ok(()));
		assert_eq!(cache.len(), 2);
		assert_eq!(cache.map.len(), 2);
		assert!(cache.map.get(&3).is_none());
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "one");

		// write 4: no eviction (capacity=3), becomes new tail
		cache.write(4, "four");
		assert_eq!(cache.len(), 3);
		assert_eq!(cache.map.len(), 3);
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "four");

		// read tail is a no-op
		assert_eq!(cache.read(&4), Some(&"four"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "two");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "four");

		// write 5: triggers eviction of LRU head ("two")
		cache.write(5, "five");
		assert_eq!(cache.len(), 3);
		assert!(cache.map.get(&2).is_none());
		assert!(cache.map.get(&1).is_some());
		assert!(cache.map.get(&4).is_some());
		assert!(cache.map.get(&5).is_some());
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "five");

		// delete tail ("five")
		assert_eq!(cache.delete(&5), Ok(()));
		assert_eq!(cache.len(), 2);
		assert!(cache.map.get(&5).is_none());
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "one");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "four");

		// clear everything
		cache.clear();
		assert_eq!(cache.len(), 0);
		assert!(cache.is_empty());
		assert!(cache.map.is_empty());
		assert_eq!(cache.head, None);
		assert_eq!(cache.tail, None);
		assert!(cache.read(&1).is_none());
		assert!(cache.read(&4).is_none());

		// behaves like fresh after clear
		cache.write(6, "six");
		assert_eq!(cache.len(), 1);
		assert!(!cache.is_empty());
		assert_eq!(cache.map.len(), 1);
		assert_eq!(cache.read(&6), Some(&"six"));
		assert_eq!(cache.items[cache.head.unwrap()].as_ref().unwrap().value, "six");
		assert_eq!(cache.items[cache.tail.unwrap()].as_ref().unwrap().value, "six");
	}
}
