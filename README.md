# Fixed Index Vector - Rust Crate

This crate provides `FixedIndexVec`, a Rust collection that functions like an array with immutable indices.
Each value is associated with a unique index upon insertion.
The value can be accessed, inserted, and removed with the index serving as the identifier.
An item cannot be modified, nor can it be replaced with another item, even after removal.

With default features, `FixedIndexVec` has no dependencies outside of the Rust standard library.

## Motivation

I was looking for a way to do simple version control within a data structure. 
The immutability of the indices in `FixedIndexVec` allows for a simple implementation of this functionality.
Items' insertion order is preserved through comparison of their indices, and the removal of an item is reflected
by indices that return `None` when accessed.

To access the "current" version of an item, simply call `FixedIndexVec::last()`.

There are other use cases for `FixedIndexVec` as well, but this is the one that motivated me to create it.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fixed_index_vec = "0.1.0"
```

Then include it in your application:

```rust
use fixed_index_vec::FixedIndexVec;
```

## Functionality

- Create a new `FixedIndexVec` with `FixedIndexVec::new()`
- Insert a new element at the end of the `FixedIndexVec` with `FixedIndexVec::push(value)`
- Remove an element at a given index with `FixedIndexVec::remove(index)`. If no element exists at that index, returns `None`.
- Return a reference to the element at a given index with `FixedIndexVec::get(index)`. If no element exists at that index, returns `None`.
- Iterate over all elements in ascending order of their indices with `FixedIndexVec::iter()`. Skips indices that do not have a corresponding value.
- Return the number of elements in the `FixedIndexVec` with `FixedIndexVec::len()`
- Check if `FixedIndexVec` contains no elements with `FixedIndexVec::is_empty()`
- Clears all values from `FixedIndexVec` with `FixedIndexVec::clear()`
- Clears all values from `FixedIndexVec` and resets the next index to 0 with `FixedIndexVec::reset()`

## Features

- `enable-serde`: Enables serialization and deserialization of `FixedIndexVec` with [Serde](https://serde.rs/). Requires `FixedIndexVec` to contain values that implement `Serialize` and `DeserializeOwned`.

## Example

```rust
use fixed_index_vec::FixedIndexVec;
let mut vec = FixedIndexVec::new();
vec.push("value1".to_string()); vec.push("value2".to_string());
assert_eq!(vec.get(0), Some(&"value1".to_string())); assert_eq!(vec.get(1), Some(&"value2".to_string()));
vec.remove(1);
assert_eq!(vec.get(1), None);
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License.

