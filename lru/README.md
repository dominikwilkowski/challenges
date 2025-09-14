```
╦   ╦═╗ ╦ ╦
║   ╠╦╝ ║ ║
╩═╝ ╩╚═ ╚═╝
```

## The Problem

Design a LRU (Least Recently Used) in-memory cache.

```rust
let mut cache = LruCache::new(3);

cache.write("a", 1);
cache.write("b", 2);
cache.write("c", 3);
assert_eq!(cache.len(), 3);
// Cache order: [c, b, a]

assert_eq!(cache.read(&"a"), Some(&1));
// Cache order: [a, c, b]

cache.write("d", 4);
assert_eq!(cache.len(), 3);
assert_eq!(cache.read(&"b"), None);
// Cache order: [d, a, c]

cache.write("a", 10);
assert_eq!(cache.read("a"), Some(&10));
assert_eq!(cache.len(), 3);
// Cache order: [a, d, c]

assert_eq!(cache.delete("c"), Some(3));
assert_eq!(cache.len(), 2);
// Cache order: [a, d]

cache.clear();
assert_eq!(cache.len(), 0);
```
