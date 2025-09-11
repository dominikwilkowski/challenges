use std::collections::HashMap;

struct Node<K, T> {
	key: K,
	value: T,
	next: Option<usize>,
	prev: Option<usize>,
}

impl<K, T> Node<K, T> {
	fn new(key: K, value: T, next: Option<usize>, prev: Option<usize>) -> Self {
		Self { key, value, next, prev }
	}
}

pub struct LruCache<K, T> {
	values: Vec<Option<Node<K, T>>>,
	map: HashMap<K, usize>,
	head: Option<usize>,
	tail: Option<usize>,
}

impl<K, T> LruCache<K, T> {
	pub fn new(size: usize) -> Self {
		Self {
			values: Vec::with_capacity(size),
			map: HashMap::with_capacity(size),
			head: None,
			tail: None,
		}
	}

	// TODO: add method, get method, re-order, remove method
}
