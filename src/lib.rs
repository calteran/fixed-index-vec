#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
use std::collections::BTreeMap;
use std::fmt::Display;

/// A fixed-size indexed vector that maps indices to values.
///
/// It provides a fixed-size vector-like data structure that can store values based on its
/// associated index.
/// Each value is associated with a unique index in the map.
/// The values can be
/// accessed, inserted, and removed using the index as the identifier.
///
/// # Examples
///
/// ```
/// use fixed_index_vec::FixedIndexVec;
///
/// let mut vec = FixedIndexVec::new();
///
/// vec.insert("value1".to_string());
/// vec.insert("value2".to_string());
///
/// assert_eq!(vec.get(0), Some(&"value1".to_string()));
/// assert_eq!(vec.get(1), Some(&"value2".to_string()));
///
/// vec.remove(1);
///
/// assert_eq!(vec.get(1), None);
/// ```
///
/// # Notes
///
/// - The `FixedIndexVec` is backed by a `BTreeMap`, so it is not as fast as a `Vec`.
/// - Index notations are supported (eg. `vec[0]`), however, accessing an index that does not
///  exist will panic.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FixedIndexVec<T> {
    map: BTreeMap<usize, T>,
    next_index: usize,
}

impl<T: Display> Display for FixedIndexVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (i, v) in self.iter() {
            s.push_str(&format!("{}: {}\n", i, v));
        }
        write!(f, "{}", s)
    }
}

impl<T> FixedIndexVec<T> {
    /// Creates an empty `FixedIndexVec`.
    ///
    /// The internal storage will not allocate until elements are pushed onto it.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    /// let mut vec: FixedIndexVec<i32> = FixedIndexVec::new();
    /// ```
    pub fn new() -> FixedIndexVec<T> {
        FixedIndexVec {
            map: BTreeMap::new(),
            next_index: 0,
        }
    }

    /// Inserts an element at the end of the `FixedIndexVec`.
    ///
    /// # Panics
    ///
    /// Panics if the `FixedIndexVec` is at capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec = FixedIndexVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec[0], 1);
    /// assert_eq!(vec[1], 2);
    /// ```
    pub fn push(&mut self, value: T) {
        self.map.insert(self.next_index, value);
        self.next_index += 1;
    }

    /// Alias for `push`.
    /// Inserts an element at the end of the `FixedIndexVec`.
    ///
    /// # Panics
    ///
    /// Panics if the `FixedIndexVec` is at capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec = FixedIndexVec::new();
    /// vec.insert(1);
    /// vec.insert(2);
    /// assert_eq!(vec[0], 1);
    /// assert_eq!(vec[1], 2);
    /// ```
    pub fn insert(&mut self, value: T) {
        self.push(value);
    }

    /// Removes the element at the given index, if it exists, returning it or `None` if it does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec = FixedIndexVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.remove(0), Some(1));
    /// assert_eq!(vec.remove(0), None);
    /// ```
    ///
    /// # Notes
    ///
    /// Unlike `Vec::remove`, this does not shift elements after the removed element.
    /// If index >= length, this returns `None`, the same as if the element did not exist.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.map.remove(&index)
    }

    /// Returns a reference to the element at the given index,
    /// if it exists, or `None` if it does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec = FixedIndexVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.get(0), Some(&1));
    /// assert_eq!(vec.get(2), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        self.map.get(&index)
    }

    /// An iterator visiting all elements in ascending order of their indices.
    /// The index is returned along with the value.
    /// The iterator skips indices that do not have a corresponding value.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec: FixedIndexVec<i32> = vec![1, 2, 3].into();
    /// vec.remove(1);
    /// let mut iter = vec.iter();
    /// assert_eq!(iter.next(), Some((0, &1)));
    /// assert_eq!(iter.next(), Some((2, &3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.map.iter().map(|(i, v)| (*i, v))
    }

    /// Returns the number of elements in the `FixedIndexVec`.
    /// This is not the same as the value of the largest index, unless no elements have been removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec: FixedIndexVec<i32> = vec![1, 2, 3].into();
    /// vec.remove(1);
    /// assert_eq!(vec.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the `FixedIndexVec` contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec: FixedIndexVec<i32> = vec![1, 2, 3].into();
    /// vec.remove(1);
    /// assert_eq!(vec.is_empty(), false);
    /// ```
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    /// let vec: FixedIndexVec<i32> = FixedIndexVec::new();
    /// assert_eq!(vec.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Clears the `FixedIndexVec`, removing all values.
    /// Keeps the allocated memory for reuse.
    /// This is equivalent to calling `remove` on every index.
    /// The next index will *not* be reset to 0.
    ///
    /// # Examples
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec: FixedIndexVec<i32> = vec![1, 2, 3].into();
    /// vec.clear();
    /// assert_eq!(vec.len(), 0);
    /// assert_eq!(vec.next_index(), 3);
    /// ```
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Clears the `FixedIndexVec`, removing all values and resetting the next index to 0.
    /// Keeps the allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec: FixedIndexVec<i32> = vec![1, 2, 3].into();
    /// vec.reset();
    /// assert_eq!(vec.len(), 0);
    /// assert_eq!(vec.next_index(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.map.clear();
        self.next_index = 0;
    }

    /// Returns the next index that will be used when inserting an element.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec: FixedIndexVec<i32> = vec![1, 2, 3].into();
    /// vec.remove(1);
    /// assert_eq!(vec.next_index(), 3);
    /// ```
    pub fn next_index(&self) -> usize {
        self.next_index
    }

    /// Returns the index and a reference to the element at the smallest populated index, or `None`
    /// if the `FixedIndexVec` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec: FixedIndexVec<i32> = vec![1, 2, 3].into();
    /// vec.remove(0);
    /// assert_eq!(vec.first(), Some((1, &2)));
    /// ```
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let vec: FixedIndexVec<i32> = FixedIndexVec::new();
    /// assert_eq!(vec.first(), None);
    pub fn first(&self) -> Option<(usize, &T)> {
        self.iter().next()
    }

    /// Returns the index and a reference to the element at the largest populated index, or `None`
    /// if the `FixedIndexVec` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let mut vec: FixedIndexVec<i32> = vec![1, 2, 3].into();
    /// vec.remove(2);
    /// assert_eq!(vec.last(), Some((1, &2)));
    /// ```
    /// ```
    /// use fixed_index_vec::FixedIndexVec;
    ///
    /// let vec: FixedIndexVec<i32> = FixedIndexVec::new();
    /// assert_eq!(vec.last(), None);
    /// ```
    pub fn last(&self) -> Option<(usize, &T)> {
        self.iter().last()
    }
}

impl<T> std::ops::Index<usize> for FixedIndexVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T> FromIterator<T> for FixedIndexVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> FixedIndexVec<T> {
        let mut map = BTreeMap::new();
        for (i, v) in iter.into_iter().enumerate() {
            map.insert(i, v);
        }
        FixedIndexVec {
            next_index: map.len(),
            map,
        }
    }
}

impl<T> From<Vec<T>> for FixedIndexVec<T> {
    fn from(vec: Vec<T>) -> FixedIndexVec<T> {
        vec.into_iter().collect()
    }
}
