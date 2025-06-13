use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use base64::Engine as _;
use base64::engine::general_purpose;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ValueError, ValueResult};

/// Binary data value type with comprehensive operations and encodings
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BinaryValue(#[cfg_attr(feature = "serde", serde(with = "serde_bytes"))] Vec<u8>);

impl BinaryValue {
    // === Constructors ===

    /// Creates a new binary value from bytes
    #[inline]
    #[must_use]
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    /// Creates an empty binary value
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Creates a binary value with specific capacity
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Creates a binary value filled with zeros
    #[must_use]
    pub fn zeros(size: usize) -> Self {
        Self(vec![0; size])
    }

    /// Creates a binary value filled with ones (0xFF)
    #[must_use]
    pub fn ones(size: usize) -> Self {
        Self(vec![0xFF; size])
    }

    /// Creates a binary value filled with a specific byte
    #[must_use]
    pub fn filled(byte: u8, size: usize) -> Self {
        Self(vec![byte; size])
    }

    // === Basic Properties ===

    /// Returns the length of the binary data in bytes
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks if the binary data is empty
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the capacity of the internal vector
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Reserves capacity for at least additional more bytes
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    /// Shrinks the capacity as much as possible
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    /// Clears all data
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    // === Data Access ===

    /// Returns a reference to the inner bytes
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns a mutable reference to the inner bytes
    #[inline]
    #[must_use]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// Consumes self and returns inner Vec<u8>
    #[inline]
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Gets a byte at the specified index
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<u8> {
        self.0.get(index).copied()
    }

    /// Safely gets a byte at the specified index
    pub fn try_get(&self, index: usize) -> ValueResult<u8> {
        self.get(index).ok_or_else(|| {
            ValueError::custom(format!(
                "Index {} out of bounds for binary data of length {}",
                index,
                self.len()
            ))
        })
    }

    /// Sets a byte at the specified index
    pub fn set(&mut self, index: usize, byte: u8) -> ValueResult<()> {
        if index >= self.len() {
            return Err(ValueError::custom(format!(
                "Index {} out of bounds for binary data of length {}",
                index,
                self.len()
            )));
        }
        self.0[index] = byte;
        Ok(())
    }

    // === Modification Operations ===

    /// Appends a byte to the end
    #[inline]
    pub fn push(&mut self, byte: u8) {
        self.0.push(byte);
    }

    /// Removes and returns the last byte
    #[inline]
    #[must_use]
    pub fn pop(&mut self) -> Option<u8> {
        self.0.pop()
    }

    /// Inserts a byte at the specified position
    pub fn insert(&mut self, index: usize, byte: u8) -> ValueResult<()> {
        if index > self.len() {
            return Err(ValueError::custom(format!("Index {index} out of bounds for insertion")));
        }
        self.0.insert(index, byte);
        Ok(())
    }

    /// Removes and returns the byte at the specified position
    pub fn remove(&mut self, index: usize) -> ValueResult<u8> {
        if index >= self.len() {
            return Err(ValueError::custom(format!("Index {index} out of bounds for removal")));
        }
        Ok(self.0.remove(index))
    }

    /// Extends with bytes from another BinaryValue
    #[inline]
    pub fn extend(&mut self, other: &Self) {
        self.0.extend_from_slice(&other.0);
    }

    /// Extends with bytes from a slice
    #[inline]
    pub fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }

    /// Appends another BinaryValue
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0);
    }

    // === Slicing and Subranges ===

    /// Returns a slice of the binary data
    pub fn slice(&self, start: usize, end: usize) -> ValueResult<Self> {
        if start > end {
            return Err(ValueError::custom(format!(
                "Invalid slice range: start ({start}) > end ({end})"
            )));
        }
        if end > self.len() {
            return Err(ValueError::custom(format!("End index {end} out of bounds")));
        }
        Ok(Self::new(self.0[start..end].to_vec()))
    }

    /// Returns the first n bytes
    #[must_use]
    pub fn take(&self, n: usize) -> Self {
        let end = n.min(self.len());
        Self::new(self.0[..end].to_vec())
    }

    /// Returns all bytes after skipping n
    #[must_use]
    pub fn skip(&self, n: usize) -> Self {
        let start = n.min(self.len());
        Self::new(self.0[start..].to_vec())
    }

    /// Splits the binary data into chunks of specified size
    pub fn chunks(&self, chunk_size: usize) -> ValueResult<Vec<Self>> {
        if chunk_size == 0 {
            return Err(ValueError::custom("Chunk size cannot be zero"));
        }

        let chunks: Vec<Self> =
            self.0.chunks(chunk_size).map(|chunk| Self::new(chunk.to_vec())).collect();

        Ok(chunks)
    }

    // === String Encodings ===

    /// Converts to base64 string using standard encoding
    #[must_use]
    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.0)
    }

    /// Converts to base64 string using URL-safe encoding
    #[must_use]
    pub fn to_base64_url(&self) -> String {
        general_purpose::URL_SAFE.encode(&self.0)
    }

    /// Converts to base64 string without padding
    #[must_use]
    pub fn to_base64_no_pad(&self) -> String {
        general_purpose::STANDARD_NO_PAD.encode(&self.0)
    }

    /// Creates from base64 string (standard encoding)
    pub fn from_base64(encoded: &str) -> ValueResult<Self> {
        general_purpose::STANDARD
            .decode(encoded)
            .map(Self::new)
            .map_err(|e| ValueError::custom(format!("Base64 decode error: {e}")))
    }

    /// Creates from base64 string (URL-safe encoding)
    pub fn from_base64_url(encoded: &str) -> ValueResult<Self> {
        general_purpose::URL_SAFE
            .decode(encoded)
            .map(Self::new)
            .map_err(|e| ValueError::custom(format!("Base64 URL decode error: {e}")))
    }

    /// Converts to hexadecimal string (lowercase)
    #[must_use]
    pub fn to_hex(&self) -> String {
        self.0.iter().map(|byte| format!("{byte:02x}")).collect::<String>()
    }

    /// Converts to hexadecimal string (uppercase)
    #[must_use]
    pub fn to_hex_upper(&self) -> String {
        self.0.iter().map(|byte| format!("{byte:02X}")).collect::<String>()
    }

    /// Creates from hexadecimal string
    pub fn from_hex(hex: &str) -> ValueResult<Self> {
        let hex = hex.trim().replace(" ", "").replace(":", "").replace("-", "");

        if hex.len() % 2 != 0 {
            return Err(ValueError::custom("Hex string must have even length"));
        }

        let mut bytes = Vec::with_capacity(hex.len() / 2);

        for i in (0..hex.len()).step_by(2) {
            let byte_str = &hex[i..i + 2];
            match u8::from_str_radix(byte_str, 16) {
                Ok(byte) => bytes.push(byte),
                Err(_) => {
                    return Err(ValueError::custom(format!("Invalid hex byte: {byte_str}")));
                },
            }
        }

        Ok(Self::new(bytes))
    }

    /// Attempts to convert to UTF-8 string
    pub fn to_utf8(&self) -> ValueResult<String> {
        String::from_utf8(self.0.clone())
            .map_err(|e| ValueError::custom(format!("Invalid UTF-8: {e}")))
    }

    /// Attempts to convert to UTF-8 string (lossy conversion)
    #[must_use]
    pub fn to_utf8_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }

    // === Bitwise Operations ===

    /// Performs bitwise AND with another BinaryValue
    pub fn bitwise_and(&self, other: &Self) -> ValueResult<Self> {
        if self.len() != other.len() {
            return Err(ValueError::custom(
                "Binary values must have the same length for bitwise operations",
            ));
        }

        let result: Vec<u8> = self.0.iter().zip(&other.0).map(|(a, b)| a & b).collect();

        Ok(Self::new(result))
    }

    /// Performs bitwise OR with another BinaryValue
    pub fn bitwise_or(&self, other: &Self) -> ValueResult<Self> {
        if self.len() != other.len() {
            return Err(ValueError::custom(
                "Binary values must have the same length for bitwise operations",
            ));
        }

        let result: Vec<u8> = self.0.iter().zip(&other.0).map(|(a, b)| a | b).collect();

        Ok(Self::new(result))
    }

    /// Performs bitwise XOR with another BinaryValue
    pub fn bitwise_xor(&self, other: &Self) -> ValueResult<Self> {
        if self.len() != other.len() {
            return Err(ValueError::custom(
                "Binary values must have the same length for bitwise operations",
            ));
        }

        let result: Vec<u8> = self.0.iter().zip(&other.0).map(|(a, b)| a ^ b).collect();

        Ok(Self::new(result))
    }

    /// Performs bitwise NOT (inverts all bits)
    #[must_use]
    pub fn bitwise_not(&self) -> Self {
        let result: Vec<u8> = self.0.iter().map(|b| !b).collect();
        Self::new(result)
    }

    /// Left shift all bytes by n bits
    #[must_use]
    pub fn left_shift(&self, n: u8) -> Self {
        if n >= 8 {
            return Self::zeros(self.len());
        }

        let mut result = vec![0u8; self.len()];
        let mut carry = 0u8;

        for (i, &byte) in self.0.iter().enumerate() {
            result[i] = (byte << n) | carry;
            carry = byte >> (8 - n);
        }

        Self::new(result)
    }

    /// Right shift all bytes by n bits
    #[must_use]
    pub fn right_shift(&self, n: u8) -> Self {
        if n >= 8 {
            return Self::zeros(self.len());
        }

        let mut result = vec![0u8; self.len()];
        let mut carry = 0u8;

        for (i, &byte) in self.0.iter().enumerate().rev() {
            result[i] = (byte >> n) | (carry << (8 - n));
            carry = byte & ((1 << n) - 1);
        }

        Self::new(result)
    }

    // === Search and Pattern Matching ===

    /// Finds the first occurrence of a byte
    #[must_use]
    pub fn find_byte(&self, byte: u8) -> Option<usize> {
        self.0.iter().position(|&b| b == byte)
    }

    /// Finds the last occurrence of a byte
    #[must_use]
    pub fn rfind_byte(&self, byte: u8) -> Option<usize> {
        self.0.iter().rposition(|&b| b == byte)
    }

    /// Finds the first occurrence of a pattern
    #[must_use]
    pub fn find_pattern(&self, pattern: &[u8]) -> Option<usize> {
        if pattern.is_empty() {
            return Some(0);
        }

        self.0.windows(pattern.len()).position(|window| window == pattern)
    }

    /// Counts occurrences of a byte
    #[must_use]
    pub fn count_byte(&self, byte: u8) -> usize {
        self.0.iter().filter(|&&b| b == byte).count()
    }

    /// Checks if the binary data contains a byte
    #[inline]
    #[must_use]
    pub fn contains_byte(&self, byte: u8) -> bool {
        self.0.contains(&byte)
    }

    /// Checks if the binary data contains a pattern
    #[must_use]
    pub fn contains_pattern(&self, pattern: &[u8]) -> bool {
        self.find_pattern(pattern).is_some()
    }

    /// Checks if the binary data starts with a pattern
    #[inline]
    #[must_use]
    pub fn starts_with(&self, pattern: &[u8]) -> bool {
        self.0.starts_with(pattern)
    }

    /// Checks if the binary data ends with a pattern
    #[inline]
    #[must_use]
    pub fn ends_with(&self, pattern: &[u8]) -> bool {
        self.0.ends_with(pattern)
    }

    // === Transformation Operations ===

    /// Reverses the byte order
    #[must_use]
    pub fn reverse(&self) -> Self {
        let mut result = self.0.clone();
        result.reverse();
        Self::new(result)
    }

    /// Reverses the byte order in place
    pub fn reverse_mut(&mut self) {
        self.0.reverse();
    }

    /// Removes duplicate consecutive bytes
    #[must_use]
    pub fn dedup(&self) -> Self {
        let mut result = self.0.clone();
        result.dedup();
        Self::new(result)
    }

    /// Concatenates with another BinaryValue
    #[must_use]
    pub fn concat(&self, other: &Self) -> Self {
        let mut result = self.0.clone();
        result.extend_from_slice(&other.0);
        Self::new(result)
    }

    /// Repeats the binary data n times
    #[must_use]
    pub fn repeat(&self, n: usize) -> Self {
        Self::new(self.0.repeat(n))
    }

    // === Statistics and Analysis ===

    /// Returns the sum of all bytes (with overflow wrapping)
    #[must_use]
    pub fn checksum_simple(&self) -> u8 {
        self.0.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
    }

    /// Returns the XOR of all bytes
    #[must_use]
    pub fn checksum_xor(&self) -> u8 {
        self.0.iter().fold(0u8, |acc, &b| acc ^ b)
    }

    /// Calculates a simple hash of the binary data
    #[must_use]
    pub fn hash_simple(&self) -> u64 {
        let mut hash = 0u64;
        for &byte in &self.0 {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }

    /// Returns statistics about byte distribution
    #[must_use]
    pub fn byte_statistics(&self) -> ByteStatistics {
        if self.is_empty() {
            return ByteStatistics::default();
        }

        let mut counts = [0usize; 256];
        let mut min = u8::MAX;
        let mut max = u8::MIN;

        for &byte in &self.0 {
            counts[byte as usize] += 1;
            min = min.min(byte);
            max = max.max(byte);
        }

        let unique_bytes = counts.iter().filter(|&&count| count > 0).count();
        let most_common = counts
            .iter()
            .enumerate()
            .max_by_key(|&(_, count)| count)
            .map(|(byte, &count)| (byte as u8, count))
            .unwrap_or((0, 0));

        ByteStatistics {
            length: self.len(),
            min_byte: min,
            max_byte: max,
            unique_bytes,
            most_common_byte: most_common.0,
            most_common_count: most_common.1,
        }
    }

    // === Compression Detection ===

    /// Estimates entropy (randomness) of the data
    #[must_use]
    pub fn entropy(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }

        let mut counts = [0usize; 256];
        for &byte in &self.0 {
            counts[byte as usize] += 1;
        }

        let length = self.len() as f64;
        let mut entropy = 0.0;

        for count in counts.iter() {
            if *count > 0 {
                let probability = *count as f64 / length;
                entropy -= probability * probability.log2();
            }
        }

        entropy
    }

    /// Estimates if the data might be compressed (high entropy)
    #[must_use]
    pub fn appears_compressed(&self) -> bool {
        self.entropy() > 7.0 // High entropy suggests compression or encryption
    }

    // === File Type Detection ===

    /// Attempts to detect file type based on magic bytes
    #[must_use]
    pub fn detect_file_type(&self) -> Option<&'static str> {
        if self.len() < 4 {
            return None;
        }

        match &self.0[..] {
            // Images
            [0xFF, 0xD8, 0xFF, ..] => Some("jpeg"),
            [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, ..] => Some("png"),
            [0x47, 0x49, 0x46, 0x38, ..] => Some("gif"),
            [0x42, 0x4D, ..] => Some("bmp"),
            [0x52, 0x49, 0x46, 0x46, _, _, _, _, 0x57, 0x45, 0x42, 0x50, ..] => Some("webp"),

            // Documents
            [0x25, 0x50, 0x44, 0x46, ..] => Some("pdf"),
            [0x50, 0x4B, 0x03, 0x04, ..] | [0x50, 0x4B, 0x05, 0x06, ..] => Some("zip"),

            // Audio
            [0x49, 0x44, 0x33, ..] => Some("mp3"),
            [0x52, 0x49, 0x46, 0x46, _, _, _, _, 0x57, 0x41, 0x56, 0x45, ..] => Some("wav"),

            // Video
            [_, _, _, _, 0x66, 0x74, 0x79, 0x70, ..] => Some("mp4"),

            // Archives
            [0x1F, 0x8B, ..] => Some("gzip"),
            [0x42, 0x5A, 0x68, ..] => Some("bzip2"),
            [0x7F, 0x45, 0x4C, 0x46, ..] => Some("elf"),

            // Executables
            [0x4D, 0x5A, ..] => Some("exe"),

            _ => None,
        }
    }
}

/// Statistics about byte distribution in binary data
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ByteStatistics {
    /// Total length of the data
    pub length: usize,
    /// Minimum byte value
    pub min_byte: u8,
    /// Maximum byte value
    pub max_byte: u8,
    /// Number of unique byte values
    pub unique_bytes: usize,
    /// Most frequently occurring byte
    pub most_common_byte: u8,
    /// Count of the most common byte
    pub most_common_count: usize,
}

impl Default for BinaryValue {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

// === Trait Implementations ===

impl fmt::Display for BinaryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base64())
    }
}

impl FromStr for BinaryValue {
    type Err = ValueError;

    fn from_str(s: &str) -> ValueResult<Self> {
        // Try to parse as hex first, then base64
        if s.chars()
            .all(|c| c.is_ascii_hexdigit() || c.is_ascii_whitespace() || c == ':' || c == '-')
        {
            Self::from_hex(s)
        } else {
            Self::from_base64(s)
        }
    }
}

impl Deref for BinaryValue {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BinaryValue {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// === From implementations ===

impl From<Vec<u8>> for BinaryValue {
    #[inline]
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl From<&[u8]> for BinaryValue {
    #[inline]
    fn from(data: &[u8]) -> Self {
        Self(data.to_vec())
    }
}

impl From<&str> for BinaryValue {
    #[inline]
    fn from(data: &str) -> Self {
        Self(data.as_bytes().to_vec())
    }
}

impl From<String> for BinaryValue {
    #[inline]
    fn from(data: String) -> Self {
        Self(data.into_bytes())
    }
}

impl From<BinaryValue> for Vec<u8> {
    #[inline]
    fn from(value: BinaryValue) -> Vec<u8> {
        value.0
    }
}

// === Collection trait implementations ===

impl FromIterator<u8> for BinaryValue {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Extend<u8> for BinaryValue {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl IntoIterator for BinaryValue {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a BinaryValue {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// === JSON conversion (feature-gated) ===

#[cfg(feature = "json")]
impl From<BinaryValue> for serde_json::Value {
    fn from(value: BinaryValue) -> serde_json::Value {
        serde_json::Value::String(value.to_base64())
    }
}

#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for BinaryValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::String(s) => Self::from_str(&s),
            serde_json::Value::Array(arr) => {
                let bytes: Result<Vec<u8>, _> = arr
                    .into_iter()
                    .map(|v| {
                        v.as_u64()
                            .and_then(|n| u8::try_from(n).ok())
                            .ok_or_else(|| ValueError::custom("Invalid byte value in array"))
                    })
                    .collect();
                Ok(Self::new(bytes?))
            },
            other => Err(ValueError::custom(format!("Cannot convert {:?} to BinaryValue", other))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        let empty = BinaryValue::empty();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);

        let zeros = BinaryValue::zeros(5);
        assert_eq!(zeros.len(), 5);
        assert_eq!(zeros.as_bytes(), &[0, 0, 0, 0, 0]);

        let ones = BinaryValue::ones(3);
        assert_eq!(ones.as_bytes(), &[0xFF, 0xFF, 0xFF]);

        let filled = BinaryValue::filled(42, 4);
        assert_eq!(filled.as_bytes(), &[42, 42, 42, 42]);
    }

    #[test]
    fn test_encoding_decoding() {
        let data = b"hello world";
        let bv = BinaryValue::from(&data[..]);

        // Base64
        let base64 = bv.to_base64();
        assert_eq!(base64, "aGVsbG8gd29ybGQ=");
        let decoded = BinaryValue::from_base64(&base64).unwrap();
        assert_eq!(bv, decoded);

        // Hex
        let hex = bv.to_hex();
        assert_eq!(hex, "68656c6c6f20776f726c64");
        let from_hex = BinaryValue::from_hex(&hex).unwrap();
        assert_eq!(bv, from_hex);

        // UTF-8
        let utf8 = bv.to_utf8().unwrap();
        assert_eq!(utf8, "hello world");
    }

    #[test]
    fn test_bitwise_operations() {
        let a = BinaryValue::from(vec![0b11110000, 0b10101010]);
        let b = BinaryValue::from(vec![0b11001100, 0b01010101]);

        let and_result = a.bitwise_and(&b).unwrap();
        assert_eq!(and_result.as_bytes(), &[0b11000000, 0b00000000]);

        let or_result = a.bitwise_or(&b).unwrap();
        assert_eq!(or_result.as_bytes(), &[0b11111100, 0b11111111]);

        let xor_result = a.bitwise_xor(&b).unwrap();
        assert_eq!(xor_result.as_bytes(), &[0b00111100, 0b11111111]);

        let not_result = a.bitwise_not();
        assert_eq!(not_result.as_bytes(), &[0b00001111, 0b01010101]);
    }

    #[test]
    fn test_searching() {
        let data = BinaryValue::from(b"hello world".to_vec());

        assert_eq!(data.find_byte(b'o'), Some(4));
        assert_eq!(data.rfind_byte(b'o'), Some(7));
        assert_eq!(data.count_byte(b'l'), 3);
        assert!(data.contains_byte(b'w'));
        assert!(!data.contains_byte(b'x'));

        assert_eq!(data.find_pattern(b"wor"), Some(6));
        assert!(data.contains_pattern(b"ello"));
        assert!(data.starts_with(b"hello"));
        assert!(data.ends_with(b"world"));
    }

    #[test]
    fn test_statistics() {
        let data = BinaryValue::from(b"aabbcc".to_vec());
        let stats = data.byte_statistics();

        assert_eq!(stats.length, 6);
        assert_eq!(stats.min_byte, b'a');
        assert_eq!(stats.max_byte, b'c');
        assert_eq!(stats.unique_bytes, 3);
        assert_eq!(stats.most_common_count, 2);
    }

    #[test]
    fn test_file_type_detection() {
        // JPEG magic bytes
        let jpeg = BinaryValue::from(vec![0xFF, 0xD8, 0xFF, 0xE0]);
        assert_eq!(jpeg.detect_file_type(), Some("jpeg"));

        // PNG magic bytes
        let png = BinaryValue::from(vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
        assert_eq!(png.detect_file_type(), Some("png"));

        // Unknown type
        let unknown = BinaryValue::from(b"just text".to_vec());
        assert_eq!(unknown.detect_file_type(), None);
    }

    #[test]
    fn test_entropy() {
        // Low entropy (repeated data)
        let low_entropy = BinaryValue::from(vec![0x00; 100]);
        assert!(low_entropy.entropy() < 1.0);

        // High entropy (random-like data)
        let high_entropy = BinaryValue::from((0..256).map(|i| i as u8).collect::<Vec<_>>());
        assert!(high_entropy.entropy() > 7.0);
    }

    #[test]
    fn test_shifts() {
        let data = BinaryValue::from(vec![0b11110000]);

        let left_shifted = data.left_shift(2);
        assert_eq!(left_shifted.as_bytes(), &[0b11000000]);

        let right_shifted = data.right_shift(2);
        assert_eq!(right_shifted.as_bytes(), &[0b00111100]);
    }

    #[test]
    fn test_checksums() {
        let data = BinaryValue::from(vec![1, 2, 3, 4]);

        assert_eq!(data.checksum_simple(), 10); // 1+2+3+4
        assert_eq!(data.checksum_xor(), 4); // 1^2^3^4
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_json_conversion() {
        let data = BinaryValue::from(b"test data".to_vec());
        let json: serde_json::Value = data.clone().into();

        match json {
            serde_json::Value::String(base64_str) => {
                let back = BinaryValue::from_base64(&base64_str).unwrap();
                assert_eq!(back, data);
            },
            _ => panic!("Expected JSON string"),
        }
    }
}
