use byteorder::{LittleEndian, ReadBytesExt};
use serde::de::{
    self, value::U64Deserializer, DeserializeSeed, EnumAccess, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use serde::Deserialize;
use std::io::{Cursor, Read};

use crate::error::StoreError;

// ===========================================================================
// Binary Deserializer
// ===========================================================================

/// A Serde deserializer that reads binary data compatible with the Haskell
/// `store` library's format (Little Endian).
///
/// This implementation is based on the analysis of the Haskell `store`
/// library's source code and decoding rules.
pub struct StoreDeserializer<R> {
    reader: R,
}

impl<R: Read> StoreDeserializer<R> {
    /// Creates a new deserializer reading from the given reader.
    pub fn new(reader: R) -> Self {
        StoreDeserializer { reader }
    }

    // These methods directly map to Haskell store's primitive decoding using LittleEndian.
    fn read_u8(&mut self) -> Result<u8, StoreError> {
        self.reader.read_u8().map_err(Into::into)
    }

    fn read_i8(&mut self) -> Result<i8, StoreError> {
        self.reader.read_i8().map_err(Into::into)
    }

    fn read_u16(&mut self) -> Result<u16, StoreError> {
        self.reader.read_u16::<LittleEndian>().map_err(Into::into)
    }

    fn read_i16(&mut self) -> Result<i16, StoreError> {
        self.reader.read_i16::<LittleEndian>().map_err(Into::into)
    }

    fn read_u32(&mut self) -> Result<u32, StoreError> {
        self.reader.read_u32::<LittleEndian>().map_err(Into::into)
    }

    fn read_i32(&mut self) -> Result<i32, StoreError> {
        self.reader.read_i32::<LittleEndian>().map_err(Into::into)
    }

    fn read_u64(&mut self) -> Result<u64, StoreError> {
        self.reader.read_u64::<LittleEndian>().map_err(Into::into)
    }

    fn read_i64(&mut self) -> Result<i64, StoreError> {
        self.reader.read_i64::<LittleEndian>().map_err(Into::into)
    }

    fn read_f32(&mut self) -> Result<f32, StoreError> {
        self.reader.read_f32::<LittleEndian>().map_err(Into::into)
    }

    fn read_f64(&mut self) -> Result<f64, StoreError> {
        self.reader.read_f64::<LittleEndian>().map_err(Into::into)
    }

    // Matches Haskell store's length decoding for collections, Text, ByteString (Word64le).
    fn read_len(&mut self) -> Result<usize, StoreError> {
        let len = self.read_u64()?;
        Ok(len as usize)
    }

    // Matches Haskell store's Text decoding (Word64le length + UTF8 bytes).
    fn read_text(&mut self) -> Result<String, StoreError> {
        let len = self.read_len()?;
        let mut buf = vec![0u8; len];
        self.reader.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| StoreError::Serde(format!("Invalid UTF-8: {}", e)))
    }

    // Matches Haskell store's ByteString decoding (Word64le length + raw bytes).
    fn read_bytes(&mut self) -> Result<Vec<u8>, StoreError> {
        let len = self.read_len()?;
        let mut buf = vec![0u8; len];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

// ===========================================================================
// Implement serde::Deserializer for StoreDeserializer
// ===========================================================================

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut StoreDeserializer<R> {
    type Error = StoreError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(StoreError::Serde(
            "deserialize_any not supported by Store format".to_string(),
        ))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let val = self.read_u8()?;
        visitor.visit_bool(val != 0)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.read_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.read_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.read_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.read_i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.read_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.read_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.read_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.read_u64()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.read_f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.read_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Char is serialized as a UTF-8 string in store
        let s = self.read_text()?;
        let mut chars = s.chars();
        let c = chars
            .next()
            .ok_or_else(|| StoreError::Serde("Empty string when deserializing char".to_string()))?;
        if chars.next().is_some() {
            return Err(StoreError::Serde(
                "Multiple characters when deserializing char".to_string(),
            ));
        }
        visitor.visit_char(c)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s = self.read_text()?;
        visitor.visit_string(s)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.read_bytes()?;
        visitor.visit_byte_buf(bytes)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let tag = self.read_u8()?;
        match tag {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(StoreError::Serde(format!(
                "Invalid option tag: {}",
                tag
            ))),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self.read_len()?;
        visitor.visit_seq(StoreSeqAccess::new(self, len))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Tuples have no length prefix in store format
        visitor.visit_seq(StoreSeqAccess::new(self, len))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Tuple structs have no length prefix in store format
        visitor.visit_seq(StoreSeqAccess::new(self, len))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self.read_len()?;
        visitor.visit_map(StoreMapAccess::new(self, len))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Structs have no length prefix in store format
        visitor.visit_seq(StoreSeqAccess::new(self, fields.len()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(StoreEnumAccess::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Used for field names, variant names, etc.
        // In Store format, we use indices, so we deserialize as u64
        let idx = self.read_u64()?;
        visitor.visit_u64(idx)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

// ===========================================================================
// Helper structs for compound types
// ===========================================================================

struct StoreSeqAccess<'a, R: Read> {
    de: &'a mut StoreDeserializer<R>,
    remaining: usize,
}

impl<'a, R: Read> StoreSeqAccess<'a, R> {
    fn new(de: &'a mut StoreDeserializer<R>, len: usize) -> Self {
        StoreSeqAccess { de, remaining: len }
    }
}

impl<'de, 'a, R: Read> SeqAccess<'de> for StoreSeqAccess<'a, R> {
    type Error = StoreError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct StoreMapAccess<'a, R: Read> {
    de: &'a mut StoreDeserializer<R>,
    remaining: usize,
}

impl<'a, R: Read> StoreMapAccess<'a, R> {
    fn new(de: &'a mut StoreDeserializer<R>, len: usize) -> Self {
        StoreMapAccess { de, remaining: len }
    }
}

impl<'de, 'a, R: Read> MapAccess<'de> for StoreMapAccess<'a, R> {
    type Error = StoreError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct StoreEnumAccess<'a, R: Read> {
    de: &'a mut StoreDeserializer<R>,
}

impl<'a, R: Read> StoreEnumAccess<'a, R> {
    fn new(de: &'a mut StoreDeserializer<R>) -> Self {
        StoreEnumAccess { de }
    }
}

impl<'de, 'a, R: Read> EnumAccess<'de> for StoreEnumAccess<'a, R> {
    type Error = StoreError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        // Read the variant index (u64 in store format)
        let idx = self.de.read_u64()?;
        let val: V::Value = seed.deserialize(U64Deserializer::<StoreError>::new(idx))?;
        Ok((val, self))
    }
}

impl<'de, 'a, R: Read> VariantAccess<'de> for StoreEnumAccess<'a, R> {
    type Error = StoreError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(&mut *self.de, len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(&mut *self.de, fields.len(), visitor)
    }
}

// ===========================================================================
// Top-level deserialization function
// ===========================================================================

/// Deserializes a value from binary data compatible with the Haskell `store` library.
///
/// Assumes little-endian and uses Word64le for lengths and enum discriminants.
pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T, StoreError>
where
    T: Deserialize<'a>,
{
    let cursor = Cursor::new(bytes);
    let mut deserializer = StoreDeserializer::new(cursor);
    T::deserialize(&mut deserializer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::to_bytes;
    use serde::{Deserialize, Serialize};
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

    #[test]
    fn test_roundtrip_primitives() {
        assert_roundtrip(&true);
        assert_roundtrip(&false);
        assert_roundtrip(&42u8);
        assert_roundtrip(&42u16);
        assert_roundtrip(&42u32);
        assert_roundtrip(&42u64);
        assert_roundtrip(&(-42i8));
        assert_roundtrip(&(-42i16));
        assert_roundtrip(&(-42i32));
        assert_roundtrip(&(-42i64));
        assert_roundtrip(&3.14f32);
        assert_roundtrip(&3.14159265f64);
    }

    #[test]
    fn test_roundtrip_string() {
        assert_roundtrip(&"Hello, World!".to_string());
        assert_roundtrip(&"".to_string());
        assert_roundtrip(&"🦀 Rust 🚀".to_string());
    }

    #[test]
    fn test_roundtrip_char() {
        assert_roundtrip(&'A');
        assert_roundtrip(&'🦀');
    }

    #[test]
    fn test_roundtrip_bytes() {
        assert_roundtrip(&vec![1u8, 2, 3, 4, 5]);
        assert_roundtrip(&vec![0u8; 100]);
        assert_roundtrip(&Vec::<u8>::new());
    }

    #[test]
    fn test_roundtrip_option() {
        assert_roundtrip(&Some(42u32));
        assert_roundtrip(&None::<u32>);
        assert_roundtrip(&Some("hello".to_string()));
        assert_roundtrip(&None::<String>);
    }

    #[test]
    fn test_roundtrip_vec() {
        assert_roundtrip(&vec![1, 2, 3, 4, 5]);
        assert_roundtrip(&Vec::<i32>::new());
        assert_roundtrip(&vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[test]
    fn test_roundtrip_array() {
        assert_roundtrip(&[1u32, 2, 3, 4, 5]);
        assert_roundtrip(&[0u8; 10]);
    }

    #[test]
    fn test_roundtrip_tuple() {
        assert_roundtrip(&(1u32, "hello".to_string(), 3.14f64));
        assert_roundtrip(&(true, false));
        assert_roundtrip(&());
    }

    #[test]
    fn test_roundtrip_struct() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        assert_roundtrip(&Point { x: 10, y: 20 });

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Nested {
            name: String,
            point: Point,
            values: Vec<i32>,
        }

        assert_roundtrip(&Nested {
            name: "test".to_string(),
            point: Point { x: 1, y: 2 },
            values: vec![1, 2, 3],
        });
    }

    #[test]
    fn test_roundtrip_tuple_struct() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Wrapper(u32, String);

        assert_roundtrip(&Wrapper(42, "hello".to_string()));
    }

    #[test]
    fn test_roundtrip_enum() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        enum TestEnum {
            A,
            B(u32),
            C { x: i32, y: i32 },
        }

        assert_roundtrip(&TestEnum::A);
        assert_roundtrip(&TestEnum::B(42));
        assert_roundtrip(&TestEnum::C { x: 10, y: 20 });
    }

    #[test]
    fn test_roundtrip_map() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 1);
        map.insert("key2".to_string(), 2);
        map.insert("key3".to_string(), 3);

        let serialized = to_bytes(&map).unwrap();
        let deserialized: HashMap<String, i32> = from_bytes(&serialized).unwrap();
        
        // Compare maps element by element since order might differ
        assert_eq!(map.len(), deserialized.len());
        for (k, v) in &map {
            assert_eq!(deserialized.get(k), Some(v));
        }
    }

    #[test]
    fn test_roundtrip_btreemap() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), 1);
        map.insert("key2".to_string(), 2);
        map.insert("key3".to_string(), 3);

        assert_roundtrip(&map);
    }

    #[test]
    fn test_roundtrip_set() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let serialized = to_bytes(&set).unwrap();
        let deserialized: HashSet<i32> = from_bytes(&serialized).unwrap();
        
        assert_eq!(set.len(), deserialized.len());
        for v in &set {
            assert!(deserialized.contains(v));
        }
    }

    #[test]
    fn test_roundtrip_btreeset() {
        let mut set = BTreeSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert_roundtrip(&set);
    }

    #[test]
    fn test_roundtrip_complex() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Complex {
            id: u64,
            name: String,
            values: Vec<f64>,
            metadata: BTreeMap<String, String>,
            flags: Option<Vec<bool>>,
        }

        let mut metadata = BTreeMap::new();
        metadata.insert("author".to_string(), "test".to_string());
        metadata.insert("version".to_string(), "1.0".to_string());

        let complex = Complex {
            id: 12345,
            name: "Test Object".to_string(),
            values: vec![1.1, 2.2, 3.3],
            metadata,
            flags: Some(vec![true, false, true]),
        };

        assert_roundtrip(&complex);
    }

    // Helper function for roundtrip testing
    fn assert_roundtrip<T>(value: &T)
    where
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        let serialized = to_bytes(value).expect("Serialization failed");
        let deserialized: T = from_bytes(&serialized).expect("Deserialization failed");
        assert_eq!(value, &deserialized, "Roundtrip failed for value: {:?}", value);
    }
}