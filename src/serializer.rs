use byteorder::{LittleEndian, WriteBytesExt};
use serde::{
    Serialize, Serializer,
    ser::{
        Error, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
};
use std::io::Write;

use crate::error::StoreError;

// ===========================================================================
// Binary Serializer
// ===========================================================================

/// A Serde serializer that produces binary data compatible with the Haskell
/// `store` library's format (Little Endian).
///
/// This implementation is based on the analysis of the Haskell `store`
/// library's source code and encoding rules.
#[derive(Debug)]
pub struct StoreSerializer {
    // The buffer where we write the bytes.
    output: Vec<u8>,
}

impl Default for StoreSerializer {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl StoreSerializer {
    /// Creates a new serializer writing to an in-memory buffer.
    #[inline]
    pub fn new() -> Self {
        StoreSerializer { output: Vec::new() }
    }

    /// Creates a new serializer with pre-allocated capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        StoreSerializer {
            output: Vec::with_capacity(capacity),
        }
    }

    /// Returns the serialized bytes.
    #[inline]
    pub fn into_inner(self) -> Vec<u8> {
        self.output
    }

    /// Returns a reference to the internal buffer.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.output
    }

    // These methods directly map to Haskell store's primitive encoding using LittleEndian.
    #[inline]
    fn write_u8(&mut self, v: u8) -> Result<(), StoreError> {
        self.output.write_u8(v).map_err(Into::into)
    }
    #[inline]
    fn write_i8(&mut self, v: i8) -> Result<(), StoreError> {
        self.output.write_i8(v).map_err(Into::into)
    }
    #[inline]
    fn write_u16(&mut self, v: u16) -> Result<(), StoreError> {
        self.output.write_u16::<LittleEndian>(v).map_err(Into::into)
    }
    #[inline]
    fn write_i16(&mut self, v: i16) -> Result<(), StoreError> {
        self.output.write_i16::<LittleEndian>(v).map_err(Into::into)
    }
    #[inline]
    fn write_u32(&mut self, v: u32) -> Result<(), StoreError> {
        self.output.write_u32::<LittleEndian>(v).map_err(Into::into)
    }
    #[inline]
    fn write_i32(&mut self, v: i32) -> Result<(), StoreError> {
        self.output.write_i32::<LittleEndian>(v).map_err(Into::into)
    }
    #[inline]
    fn write_u64(&mut self, v: u64) -> Result<(), StoreError> {
        self.output.write_u64::<LittleEndian>(v).map_err(Into::into)
    }
    #[inline]
    fn write_i64(&mut self, v: i64) -> Result<(), StoreError> {
        self.output.write_i64::<LittleEndian>(v).map_err(Into::into)
    }
    #[inline]
    fn write_f32(&mut self, v: f32) -> Result<(), StoreError> {
        self.output.write_f32::<LittleEndian>(v).map_err(Into::into)
    }
    #[inline]
    fn write_f64(&mut self, v: f64) -> Result<(), StoreError> {
        self.output.write_f64::<LittleEndian>(v).map_err(Into::into)
    }

    // Matches Haskell store's length encoding for collections, Text, ByteString (Word64le).
    #[inline]
    fn write_len(&mut self, len: usize) -> Result<(), StoreError> {
        self.write_u64(len as u64)
    }

    // Matches Haskell store's Text encoding (Word64le length + UTF8 bytes).
    #[inline]
    fn write_text(&mut self, s: &str) -> Result<(), StoreError> {
        self.write_len(s.len())?;
        self.output.write_all(s.as_bytes())?;
        Ok(())
    }

    // Matches Haskell store's ByteString encoding (Word64le length + raw bytes).
    #[inline]
    fn write_bytes(&mut self, b: &[u8]) -> Result<(), StoreError> {
        self.write_len(b.len())?;
        self.output.write_all(b)?;
        Ok(())
    }
}

// ===========================================================================
// Implement serde::Serializer for &mut BinarySerializer
// ===========================================================================

impl Serializer for &mut StoreSerializer {
    type Ok = ();
    type Error = StoreError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // Primitive types match Haskell store's fixed-size LE encoding.
    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.write_u8(v as u8)
    }
    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_i8(v)
    }
    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_i16(v)
    }
    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_i32(v)
    }
    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_i64(v)
    }
    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_u8(v)
    }
    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_u16(v)
    }
    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_u32(v)
    }
    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.write_u64(v)
    }
    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.write_f32(v)
    }
    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.write_f64(v)
    }

    // Haskell store encodes Text (UTF-8) with a Word64le length prefix.
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        // Serialize char as a 1-char string (UTF-8).
        let mut buf = [0; 4];
        self.write_text(v.encode_utf8(&mut buf))
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write_text(v)
    }
    // Haskell store encodes ByteString with a Word64le length prefix.
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.write_bytes(v)
    }

    // Haskell store encodes Maybe with a Word8 tag (0/1) followed by value if Some.
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.write_u8(0)
    } // 0x00 for Nothing
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.write_u8(1)?; // 0x01 for Just
        value.serialize(self)
    }

    // Unit types encoding not explicitly detailed in store product/sum,
    // serialize as nothing.
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    // Haskell store encodes enum variants with a Word64le discriminant (index),
    // followed by variant data if any. Unit variants have no data.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant_name: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.write_u64(variant_index as u64) // Discriminant as Word64le
    }

    // Haskell store encodes newtype structs by serializing the inner value.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Haskell store encodes newtype variants with Word64le discriminant + inner value.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant_name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.write_u64(variant_index as u64)?; // Discriminant as Word64le
        value.serialize(self)
    }

    // Collections (Vec, Set, Array, Slice). Haskell store encodes with Word64le length + elements.
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.ok_or_else(|| StoreError::custom("Sequence length must be known"))?;
        self.write_len(len)?; // Length prefix as u64 LE
        Ok(self)
    }

    // Tuples and Tuple Structs. Haskell store *encodes products* (structs/tuples)
    // by serializing fields sequentially *without* a leading length count for the product itself.
    // The length is implicit from the schema.
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        // No length prefix for tuples/products in Haskell store.
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize, // Length is implicit, not written for products
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        // No length prefix for tuple structs/products in Haskell store.
        Ok(self)
    }

    // Tuple Variants. Haskell store encodes with Word64le discriminant + tuple elements.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant_name: &'static str,
        _len: usize, // Length of the tuple part is implicit, not written here
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.write_u64(variant_index as u64)?; // Discriminant as Word64le
        Ok(self)
    }

    // Maps. Haskell store encodes with Word64le length + key/value pairs.
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = len.ok_or_else(|| StoreError::custom("Map length must be known"))?;
        self.write_len(len)?; // Length prefix as u64 LE
        Ok(self)
    }

    // Structs. Haskell store encodes products (structs) by serializing fields
    // sequentially *without* a leading length count for the struct itself.
    // The number of fields is implicit from the schema.
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize, // Length is implicit, not written for products
    ) -> Result<Self::SerializeStruct, Self::Error> {
        // No length prefix for structs/products in Haskell store.
        Ok(self)
    }

    // Struct Variants. Haskell store encodes with Word64le discriminant + struct fields.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant_name: &'static str,
        _len: usize, // Length of the struct part is implicit, not written here
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.write_u64(variant_index as u64)?; // Discriminant as Word64le
        Ok(self)
    }
}

// ===========================================================================
// Implement helper traits for compound types (Seq, Tuple, Map, Struct)
//
// These traits handle the serialization of elements/fields *after* the
// length (if any) or discriminant has been written by the main serialize_* method.
// For Haskell store, the elements/fields are just serialized sequentially.
// ===========================================================================

impl SerializeSeq for &mut StoreSerializer {
    type Ok = ();
    type Error = StoreError;
    
    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }
    
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeTuple for &mut StoreSerializer {
    type Ok = ();
    type Error = StoreError;
    
    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }
    
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeTupleStruct for &mut StoreSerializer {
    type Ok = ();
    type Error = StoreError;
    
    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }
    
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeTupleVariant for &mut StoreSerializer {
    type Ok = ();
    type Error = StoreError;
    
    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }
    
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeMap for &mut StoreSerializer {
    type Ok = ();
    type Error = StoreError;
    
    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }
    
    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }
    
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeStruct for &mut StoreSerializer {
    type Ok = ();
    type Error = StoreError;
    
    #[inline]
    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // Fields are serialized sequentially without names or delimiters.
        value.serialize(&mut **self)
    }
    
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl SerializeStructVariant for &mut StoreSerializer {
    type Ok = ();
    type Error = StoreError;
    
    #[inline]
    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // Fields are serialized sequentially without names or delimiters.
        value.serialize(&mut **self)
    }
    
    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// ===========================================================================
// Top-level serialization function
// ===========================================================================

/// Serializes a value into a binary format compatible with the Haskell `store` library.
///
/// Assumes little-endian and uses Word64le for lengths and enum discriminants.
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>, StoreError>
where
    T: Serialize,
{
    let mut serializer = StoreSerializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::collections::{HashMap, HashSet};

    #[derive(Serialize, Debug, PartialEq)]
    struct TestStruct {
        a: u32,
        b: String,
        c: Vec<i16>,
        d: bool,
    }

    #[derive(Serialize, Debug, PartialEq)]
    struct NestedStruct {
        name: String,
        data: TestStruct,
    }

    // Tuple struct - should serialize without a leading length in store format
    #[derive(Serialize, Debug, PartialEq)]
    struct Point(f32, f32);

    #[derive(Serialize, Debug, PartialEq)]
    struct TestOption {
        opt_val: Option<f32>,
        opt_none: Option<i32>,
    }

    #[derive(Serialize, Debug, PartialEq)]
    enum TestEnum {
        A,                    // Unit variant, index 0
        B(u32),               // Newtype variant, index 1
        C { x: i32, y: i32 }, // Struct variant, index 2
    }

    // Helper to convert little-endian bytes for testing (Haskell store is LE)
    fn u32_le(v: u32) -> Vec<u8> {
        v.to_le_bytes().to_vec()
    }
    fn u64_le(v: u64) -> Vec<u8> {
        v.to_le_bytes().to_vec()
    }
    fn u16_le(v: u16) -> Vec<u8> {
        v.to_le_bytes().to_vec()
    }
    fn i16_le(v: i16) -> Vec<u8> {
        v.to_le_bytes().to_vec()
    }
    fn i32_le(v: i32) -> Vec<u8> {
        v.to_le_bytes().to_vec()
    } // Added i32 for struct variant test
    fn f32_le(v: f32) -> Vec<u8> {
        v.to_le_bytes().to_vec()
    }
    fn bool_byte(v: bool) -> u8 {
        v as u8
    }

    #[test]
    fn test_primitives() {
        assert_eq!(to_bytes(&5_u8).unwrap(), vec![5]);
        assert_eq!(to_bytes(&-10_i8).unwrap(), vec![0xf6]); // -10 in 2's complement
        assert_eq!(to_bytes(&1000_u16).unwrap(), u16_le(1000));
        assert_eq!(to_bytes(&-2000_i16).unwrap(), i16_le(-2000));
        assert_eq!(to_bytes(&1_000_000_u32).unwrap(), u32_le(1_000_000));
        assert_eq!(
            to_bytes(&-1_000_000_i32).unwrap(),
            (-1_000_000_i32).to_le_bytes().to_vec()
        );
        assert_eq!(
            to_bytes(&1_000_000_000_000_u64).unwrap(),
            u64_le(1_000_000_000_000)
        );
        assert_eq!(
            to_bytes(&-1_000_000_000_000_i64).unwrap(),
            (-1_000_000_000_000_i64).to_le_bytes().to_vec()
        );
        assert_eq!(to_bytes(&3.14_f32).unwrap(), f32_le(3.14));
        assert_eq!(
            to_bytes(&3.1415926535_f64).unwrap(),
            (3.1415926535_f64).to_le_bytes().to_vec()
        );
        assert_eq!(to_bytes(&true).unwrap(), vec![bool_byte(true)]);
        assert_eq!(to_bytes(&false).unwrap(), vec![bool_byte(false)]);
    }

    #[test]
    fn test_string() {
        // Valid UTF-8 string (Text in Haskell store)
        let s = String::from("hello");
        let mut expected = u64_le(s.len() as u64); // Length prefix as u64 LE
        expected.extend_from_slice(s.as_bytes());
        assert_eq!(to_bytes(&s).unwrap(), expected);

        // String with non-ASCII chars (valid UTF-8)
        let s2 = String::from("你好"); // 6 bytes in UTF-8
        let mut expected2 = u64_le(s2.len() as u64);
        expected2.extend_from_slice(s2.as_bytes());
        assert_eq!(to_bytes(&s2).unwrap(), expected2);

        // char serializes as a 1-char string (Text)
        let c = '€'; // 3 bytes in UTF-8
        let mut expected_char = u64_le(c.len_utf8() as u64);
        let mut buf = [0; 4];
        expected_char.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
        assert_eq!(to_bytes(&c).unwrap(), expected_char);
    }

    #[test]
    fn test_bytes() {
        // Corresponds to ByteString in Haskell store
        let b = vec![0x01, 0x02, 0x03, 0xff];
        let mut expected = u64_le(b.len() as u64); // Length prefix as u64 LE
        expected.extend_from_slice(&b);
        assert_eq!(to_bytes(&b).unwrap(), expected);
    }

    #[test]
    fn test_vector() {
        // Corresponds to Vector/List in Haskell store
        let v = vec![1_u32, 2_u32, 3_u32]; // Vector of u32
        let mut expected = u64_le(v.len() as u64); // Length prefix as u64 LE
        expected.extend_from_slice(&u32_le(1));
        expected.extend_from_slice(&u32_le(2));
        expected.extend_from_slice(&u32_le(3));
        assert_eq!(to_bytes(&v).unwrap(), expected);

        let v2: Vec<String> = vec!["a".into(), "bb".into()]; // Vector of strings (Text)
        let mut expected2 = u64_le(v2.len() as u64); // Vector length
        expected2.extend_from_slice(&u64_le("a".len() as u64)); // String 1 length
        expected2.extend_from_slice("a".as_bytes()); // String 1 bytes
        expected2.extend_from_slice(&u64_le("bb".len() as u64)); // String 2 length
        expected2.extend_from_slice("bb".as_bytes()); // String 2 bytes
        assert_eq!(to_bytes(&v2).unwrap(), expected2);
    }

    #[test]
    fn test_array() {
        // Fixed-size array serializes like a sequence/vector
        let a: [u32; 3] = [1, 2, 3]; // Array of u32
        let mut expected: Vec<u8> = Vec::new();
        expected.extend_from_slice(&u32_le(1));
        expected.extend_from_slice(&u32_le(2));
        expected.extend_from_slice(&u32_le(3));
        assert_eq!(to_bytes(&a).unwrap(), expected);
    }

    #[test]
    fn test_set() {
        // Set serializes like a sequence/vector (order might differ)
        let mut s: HashSet<u32> = HashSet::new();
        s.insert(42);

        // Serde's default Set serialization iterates in sorted order.
        let elements: Vec<u32> = s.iter().map(ToOwned::to_owned).collect();

        let mut expected = u64_le(elements.len() as u64); // Length prefix
        for elem in elements {
            expected.extend_from_slice(&u32_le(elem));
        }
        assert_eq!(to_bytes(&s).unwrap(), expected);
    }

    #[test]
    fn test_map() {
        // Map serializes with length + key/value pairs
        let mut m: HashMap<String, u32> = HashMap::new();
        m.insert("42".into(), 42);

        // Serde's default Map serialization iterates in sorted order by key.
        let entries: Vec<(String, u32)> = m
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();

        let mut expected = u64_le(entries.len() as u64); // Length prefix
        for (key, value) in entries {
            // Key (String/Text)
            expected.extend_from_slice(&u64_le(key.len() as u64));
            expected.extend_from_slice(key.as_bytes());
            // Value (u32)
            expected.extend_from_slice(&u32_le(value));
        }
        assert_eq!(to_bytes(&m).unwrap(), expected);
    }

    #[test]
    fn test_struct() {
        // Structs (Products) serialize fields sequentially WITHOUT leading length
        let s = TestStruct {
            a: 10,
            b: "hello".into(),
            c: vec![-1, -2], // Vec will include its own length prefix
            d: true,
        };

        let mut expected: Vec<u8> = Vec::new();
        // a: u32
        expected.extend_from_slice(&u32_le(10));
        // b: String (length + bytes)
        expected.extend_from_slice(&u64_le("hello".len() as u64));
        expected.extend_from_slice("hello".as_bytes());
        // c: Vec<i16> (length + elements)
        expected.extend_from_slice(&u64_le(2 as u64)); // Vec length
        expected.extend_from_slice(&i16_le(-1));
        expected.extend_from_slice(&i16_le(-2));
        // d: bool (as u8)
        expected.push(bool_byte(true));

        assert_eq!(to_bytes(&s).unwrap(), expected);
    }

    #[test]
    fn test_nested_struct() {
        // Nested Structs (Products) serialize fields sequentially WITHOUT leading length
        let ns = NestedStruct {
            name: "outer".into(), // String will include its length
            data: TestStruct {
                // Inner struct fields serialised sequentially
                a: 10,
                b: "hello".into(), // Inner String will include its length
                c: vec![-1, -2],   // Inner Vec will include its length
                d: true,
            },
        };

        let mut expected: Vec<u8> = Vec::new();
        // name: String
        expected.extend_from_slice(&u64_le("outer".len() as u64));
        expected.extend_from_slice("outer".as_bytes());
        // data: TestStruct (fields serialized sequentially)
        // data.a: u32
        expected.extend_from_slice(&u32_le(10));
        // data.b: String
        expected.extend_from_slice(&u64_le("hello".len() as u64));
        expected.extend_from_slice("hello".as_bytes());
        // data.c: Vec<i16>
        expected.extend_from_slice(&u64_le(2 as u64)); // Vec length
        expected.extend_from_slice(&i16_le(-1));
        expected.extend_from_slice(&i16_le(-2));
        // data.d: bool
        expected.push(bool_byte(true));

        assert_eq!(to_bytes(&ns).unwrap(), expected);
    }

    #[test]
    fn test_tuple() {
        // Tuples (Products) serialize elements sequentially WITHOUT leading length
        let t = (1_u32, "two"); // Tuple of u32 and &str (Text)

        let mut expected: Vec<u8> = Vec::new();
        // 1: u32
        expected.extend_from_slice(&u32_le(1));
        // "two": &str (length + bytes)
        expected.extend_from_slice(&u64_le("two".len() as u64));
        expected.extend_from_slice("two".as_bytes());

        assert_eq!(to_bytes(&t).unwrap(), expected);
    }

    #[test]
    fn test_tuple_struct() {
        // Tuple Structs (Products) serialize elements sequentially WITHOUT leading length
        let p = Point(1.23_f32, 4.56_f32); // Tuple struct of f32, f32

        let mut expected: Vec<u8> = Vec::new();
        // 1.23: f32
        expected.extend_from_slice(&f32_le(1.23));
        // 4.56: f32
        expected.extend_from_slice(&f32_le(4.56));

        assert_eq!(to_bytes(&p).unwrap(), expected);
    }

    #[test]
    fn test_optional() {
        // Option (Maybe) serializes with Word8 tag (0/1) + value if Some
        let opt_some = TestOption {
            opt_val: Some(1.23_f32),
            opt_none: None,
        };

        let mut expected_some: Vec<u8> = Vec::new();
        // opt_val: Option<f32> -> Some(f32)
        expected_some.push(bool_byte(true)); // Presence byte 0x01
        expected_some.extend_from_slice(&f32_le(1.23)); // Value
        // opt_none: Option<i32> -> None
        expected_some.push(bool_byte(false)); // Presence byte 0x00

        assert_eq!(to_bytes(&opt_some).unwrap(), expected_some);

        let opt_none_only = TestOption {
            opt_val: None,
            opt_none: None,
        };

        let mut expected_none: Vec<u8> = Vec::new();
        // opt_val: Option<f32> -> None
        expected_none.push(bool_byte(false)); // Presence byte 0x00
        // opt_none: Option<i32> -> None
        expected_none.push(bool_byte(false)); // Presence byte 0x00

        assert_eq!(to_bytes(&opt_none_only).unwrap(), expected_none);
    }

    #[test]
    fn test_enum() {
        // Enums (Sums) serialize with Word64le discriminant + variant data (if any)

        // Unit variant
        let e_a = TestEnum::A;
        let expected_a = u64_le(0); // Variant index 0 as u64 LE
        assert_eq!(to_bytes(&e_a).unwrap(), expected_a);

        // Newtype variant
        let e_b = TestEnum::B(42_u32);
        let mut expected_b = u64_le(1); // Variant index 1 as u64 LE
        expected_b.extend_from_slice(&u32_le(42)); // Wrapped value (u32 LE)
        assert_eq!(to_bytes(&e_b).unwrap(), expected_b);

        // Struct variant
        let e_c = TestEnum::C { x: 1, y: 2 };
        let mut expected_c = u64_le(2); // Variant index 2 as u64 LE
        expected_c.extend_from_slice(&i32_le(1)); // Field x (i32 LE)
        expected_c.extend_from_slice(&i32_le(2)); // Field y (i32 LE)
        assert_eq!(to_bytes(&e_c).unwrap(), expected_c);
    }
}
