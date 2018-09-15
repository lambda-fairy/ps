# The Philosopher's Stone

This is a proof-of-concept library
that lets you cast between arbitrary types
in a safe way.
In other words, it's a safe [`std::mem::transmute`][transmute]!

[transmute]: https://doc.rust-lang.org/std/mem/fn.transmute.html

## Example

```rust
let mut string_map: HashMap<u32, String> = HashMap::new();
string_map.insert(42, "world".into());

let bytes_map: HashMap<u32, Vec<u8>> =
    ps::hash_map_values(ps::string_bytes()).cast(string_map);

assert_eq!(map[42], b"world");
```
