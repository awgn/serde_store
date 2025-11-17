//! Helpers for serializing and deserializing ordered maps (BTreeMap) with
//! Haskell store compatibility.
//!
//! Haskell's `store` library (>= 0.4) uses a magic marker for `Map` and `IntMap`
//! to indicate that entries are stored in ascending order. This module provides
//! helper functions to maintain compatibility with that format.
//!
//! # Usage
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use std::collections::BTreeMap;
//! use serde_store::ordered_map::OrderedMap;
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     // Use the wrapper type
//!     my_map: OrderedMap<String, i32>,
//! }
//! ```

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::hash::Hash;

/// Magic marker that indicates a map is stored in ascending order.
/// This value matches the Haskell store library's `markMapPokedInAscendingOrder`.
///
/// See: https://github.com/fpco/store/issues/97
pub const ASCENDING_MAP_MARKER: u32 = 1217678090;

/// Wrapper type for BTreeMap that serializes with the Haskell store ascending order marker.
///
/// This type automatically adds the magic marker when serializing and validates it when
/// deserializing, ensuring compatibility with Haskell's `Map` and `IntMap` types.
///
/// # Example
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use serde_store::ordered_map::OrderedMap;
/// use serde_store::{to_bytes, from_bytes};
/// use std::collections::BTreeMap;
///
/// #[derive(Serialize, Deserialize)]
/// struct Config {
///     settings: OrderedMap<String, i32>,
/// }
///
/// let mut config = Config {
///     settings: OrderedMap::new(),
/// };
/// config.settings.insert("debug".to_string(), 1);
///
/// let bytes = to_bytes(&config).unwrap();
/// let decoded: Config = from_bytes(&bytes).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderedMap<K, V>(pub BTreeMap<K, V>);

impl<K, V> OrderedMap<K, V> {
    /// Create a new empty ordered map.
    #[inline]
    pub fn new() -> Self {
        OrderedMap(BTreeMap::new())
    }

    /// Get a reference to the inner BTreeMap.
    #[inline]
    pub fn inner(&self) -> &BTreeMap<K, V> {
        &self.0
    }

    /// Get a mutable reference to the inner BTreeMap.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut BTreeMap<K, V> {
        &mut self.0
    }

    /// Convert into the inner BTreeMap.
    #[inline]
    pub fn into_inner(self) -> BTreeMap<K, V> {
        self.0
    }

    /// Insert a key-value pair into the map.
    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    where
        K: Ord,
    {
        self.0.insert(k, v)
    }

    /// Get a value from the map.
    #[inline]
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.0.get(k)
    }

    /// Get the number of entries in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the map is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<K, V> Default for OrderedMap<K, V> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> From<BTreeMap<K, V>> for OrderedMap<K, V> {
    #[inline]
    fn from(map: BTreeMap<K, V>) -> Self {
        OrderedMap(map)
    }
}

impl<K, V> From<OrderedMap<K, V>> for BTreeMap<K, V> {
    #[inline]
    fn from(ordered: OrderedMap<K, V>) -> Self {
        ordered.0
    }
}

impl<K, V> std::ops::Deref for OrderedMap<K, V> {
    type Target = BTreeMap<K, V>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> std::ops::DerefMut for OrderedMap<K, V> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> Serialize for OrderedMap<K, V>
where
    K: Serialize + Ord,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;

        // Serialize as a tuple: (marker, map)
        // This ensures the marker is written before the map in the binary format
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&ASCENDING_MAP_MARKER)?;
        tuple.serialize_element(&self.0)?;
        tuple.end()
    }
}

impl<'de, K, V> Deserialize<'de> for OrderedMap<K, V>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, SeqAccess, Visitor};
        use std::fmt;
        use std::marker::PhantomData;

        struct OrderedMapVisitor<K, V> {
            marker: PhantomData<(K, V)>,
        }

        impl<'de, K, V> Visitor<'de> for OrderedMapVisitor<K, V>
        where
            K: Deserialize<'de> + Ord,
            V: Deserialize<'de>,
        {
            type Value = OrderedMap<K, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an ordered map with ascending order marker")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                // Read the magic marker
                let marker: u32 = seq
                    .next_element()?
                    .ok_or_else(|| Error::custom("missing ascending map marker"))?;

                if marker != ASCENDING_MAP_MARKER {
                    return Err(Error::custom(format!(
                        "invalid ascending map marker: expected {}, got {}",
                        ASCENDING_MAP_MARKER, marker
                    )));
                }

                // Read the actual map
                let map: BTreeMap<K, V> = seq
                    .next_element()?
                    .ok_or_else(|| Error::custom("missing map data after marker"))?;

                Ok(OrderedMap(map))
            }
        }

        deserializer.deserialize_tuple(
            2,
            OrderedMapVisitor {
                marker: PhantomData,
            },
        )
    }
}

impl<K, V> FromIterator<(K, V)> for OrderedMap<K, V>
where
    K: Ord,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        OrderedMap(BTreeMap::from_iter(iter))
    }
}

impl<'a, K, V> IntoIterator for &'a OrderedMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::collections::btree_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<K, V> IntoIterator for OrderedMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::btree_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_bytes, to_bytes};

    #[test]
    fn test_ordered_map_wrapper() {
        let mut map = OrderedMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("c".to_string(), 3);

        let bytes = to_bytes(&map).unwrap();
        let decoded: OrderedMap<String, i32> = from_bytes(&bytes).unwrap();

        assert_eq!(map, decoded);
    }

    #[test]
    fn test_marker_in_output() {
        let mut map = OrderedMap::new();
        map.insert("test".to_string(), 42);

        let bytes = to_bytes(&map).unwrap();

        // The marker should be in the first 4 bytes (as u32 little-endian)
        assert!(bytes.len() >= 4);
        let marker = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        assert_eq!(marker, ASCENDING_MAP_MARKER);
    }

    #[test]
    fn test_empty_ordered_map() {
        let map: OrderedMap<String, i32> = OrderedMap::new();
        let bytes = to_bytes(&map).unwrap();
        let decoded: OrderedMap<String, i32> = from_bytes(&bytes).unwrap();
        assert_eq!(map, decoded);
    }

    #[test]
    fn test_ordered_map_deref() {
        let mut map = OrderedMap::new();
        map.insert("key".to_string(), 100);

        // Test Deref
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key"), Some(&100));
        assert!(map.contains_key("key"));
    }

    #[test]
    fn test_ordered_map_from_btreemap() {
        let mut btree = BTreeMap::new();
        btree.insert(1, "one");
        btree.insert(2, "two");

        let ordered: OrderedMap<i32, &str> = OrderedMap::from(btree.clone());
        assert_eq!(ordered.len(), 2);
        assert_eq!(ordered.get(&1), Some(&"one"));
    }

    #[test]
    fn test_roundtrip_nested() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Container {
            id: u64,
            data: OrderedMap<String, Vec<i32>>,
        }

        let mut data = OrderedMap::new();
        data.insert("nums".to_string(), vec![1, 2, 3]);
        data.insert("vals".to_string(), vec![4, 5, 6]);

        let container = Container { id: 42, data };
        let bytes = to_bytes(&container).unwrap();
        let decoded: Container = from_bytes(&bytes).unwrap();

        assert_eq!(container, decoded);
    }

    #[test]
    fn test_from_iter() {
        let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
        let map: OrderedMap<&str, i32> = pairs.into_iter().collect();
        
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("b"), Some(&2));
    }
}