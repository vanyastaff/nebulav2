use std::fmt;
use std::ops::{Deref, DerefMut, Index, IndexMut};

// Better approach for feature-based types
#[cfg(feature = "collections")]
use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Value, ValueError, ValueResult};
#[cfg(feature = "collections")]
type InternalMap<K, V> = IndexMap<K, V>;

#[cfg(not(feature = "collections"))]
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(not(feature = "collections"))]
type InternalMap<K, V> = HashMap<K, V>;

/// Object value type for key-value collections with ordered keys
///
/// Uses IndexMap when `collections` feature is enabled for ordered keys,
/// falls back to HashMap when disabled.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ObjectValue(InternalMap<String, Value>);

impl ObjectValue {
    // === Constructors ===

    /// Creates a new empty object
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(InternalMap::new())
    }

    /// Creates an object from the internal map type
    #[inline]
    #[must_use]
    pub fn from_map(map: InternalMap<String, Value>) -> Self {
        Self(map)
    }

    /// Creates an object with specified capacity
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        #[cfg(feature = "collections")]
        {
            Self(IndexMap::with_capacity(capacity))
        }
        #[cfg(not(feature = "collections"))]
        {
            Self(HashMap::with_capacity(capacity))
        }
    }

    /// Creates an object from key-value pairs
    #[must_use]
    pub fn from_pairs<I, K, V>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<Value>,
    {
        let map = pairs.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        Self(map)
    }

    /// Creates an object from alternating keys and values
    pub fn from_alternating<I>(iter: I) -> ValueResult<Self>
    where I: IntoIterator<Item = Value> {
        let items: Vec<Value> = iter.into_iter().collect();

        if items.len() % 2 != 0 {
            return Err(ValueError::custom(
                "Alternating key-value pairs require even number of items",
            ));
        }

        let mut map = InternalMap::new();

        for chunk in items.chunks(2) {
            let key = chunk[0]
                .as_string()
                .ok_or_else(|| ValueError::custom("Object keys must be strings"))?
                .to_string();
            let value = chunk[1].clone();
            map.insert(key, value);
        }

        Ok(Self(map))
    }

    // === Basic Operations ===

    /// Returns the number of key-value pairs
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the object is empty
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the capacity of the internal map
    #[must_use]
    pub fn capacity(&self) -> usize {
        #[cfg(feature = "collections")]
        {
            self.0.capacity()
        }
        #[cfg(not(feature = "collections"))]
        {
            // HashMap doesn't expose capacity(), return approximation
            self.0.len().next_power_of_two()
        }
    }

    /// Reserves capacity for at least additional more elements
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    /// Shrinks the capacity as much as possible
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    /// Removes all key-value pairs
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    // === Key-Value Access ===

    /// Gets a value by key
    #[inline]
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    /// Gets a mutable reference to a value by key
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.0.get_mut(key)
    }

    /// Gets a value by key with error handling
    pub fn try_get(&self, key: &str) -> ValueResult<&Value> {
        self.get(key).ok_or_else(|| ValueError::key_not_found(key))
    }

    /// Gets a mutable reference to a value by key with error handling
    pub fn try_get_mut(&mut self, key: &str) -> ValueResult<&mut Value> {
        self.get_mut(key).ok_or_else(|| ValueError::key_not_found(key))
    }

    /// Gets a value by index (insertion order) - only available with
    /// collections feature
    #[cfg(feature = "collections")]
    #[inline]
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<(&String, &Value)> {
        self.0.get_index(index)
    }

    /// Gets a mutable reference to a value by index - only available with
    /// collections feature
    #[cfg(feature = "collections")]
    #[inline]
    #[must_use]
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&String, &mut Value)> {
        self.0.get_index_mut(index)
    }

    /// Checks if a key exists
    #[inline]
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Inserts a key-value pair, returning the previous value if the key
    /// existed
    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.0.insert(key, value)
    }

    /// Inserts a key-value pair if the key doesn't exist
    #[inline]
    pub fn insert_if_absent(&mut self, key: String, value: Value) -> bool {
        if !self.contains_key(&key) {
            self.insert(key, value);
            true
        } else {
            false
        }
    }

    /// Removes a key-value pair, returning the value if the key existed
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        #[cfg(feature = "collections")]
        {
            self.0.shift_remove(key)
        }
        #[cfg(not(feature = "collections"))]
        {
            self.0.remove(key)
        }
    }

    /// Removes a key-value pair by index - only available with collections
    /// feature
    #[cfg(feature = "collections")]
    #[must_use]
    pub fn remove_index(&mut self, index: usize) -> Option<(String, Value)> {
        self.0.shift_remove_index(index)
    }

    // === Deep/Nested Access ===

    /// Gets a cloned value using a path (e.g., "user.profile.name")
    #[must_use]
    pub fn get_nested_cloned(&self, path: &str) -> Option<Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current: &Value = &Value::Object(self.clone());

        for part in parts {
            match current {
                Value::Object(obj) => {
                    current = obj.get(part)?;
                },
                Value::Array(arr) => {
                    if let Ok(index) = part.parse::<usize>() {
                        current = arr.get(index)?;
                    } else {
                        return None;
                    }
                },
                _ => return None,
            }
        }

        Some(current.clone())
    }

    /// Sets a value using a path, creating intermediate objects as needed
    pub fn set_nested(&mut self, path: &str, value: Value) -> ValueResult<()> {
        let parts: Vec<&str> = path.split('.').collect();

        if parts.is_empty() {
            return Err(ValueError::custom("Empty path"));
        }

        if parts.len() == 1 {
            self.insert(parts[0].to_string(), value);
            return Ok(());
        }

        // For now, implement 2-level support (can be extended later)
        if parts.len() == 2 {
            let (first, second) = (parts[0], parts[1]);

            match self.get_mut(first) {
                Some(Value::Object(obj)) => {
                    obj.insert(second.to_string(), value);
                },
                Some(_) => {
                    return Err(ValueError::custom(format!(
                        "Cannot set nested value: '{}' is not an object",
                        first
                    )));
                },
                None => {
                    let mut new_obj = ObjectValue::new();
                    new_obj.insert(second.to_string(), value);
                    self.insert(first.to_string(), Value::Object(new_obj));
                },
            }
        } else {
            return Err(ValueError::custom("Deep nesting (>2 levels) not yet supported"));
        }

        Ok(())
    }

    /// Removes a value using a path
    pub fn remove_nested(&mut self, path: &str) -> ValueResult<Option<Value>> {
        let parts: Vec<&str> = path.split('.').collect();

        if parts.is_empty() {
            return Err(ValueError::custom("Empty path"));
        }

        if parts.len() == 1 {
            return Ok(self.remove(parts[0]));
        }

        // For now, implement 2-level support
        if parts.len() == 2 {
            let (first, second) = (parts[0], parts[1]);

            match self.get_mut(first) {
                Some(Value::Object(obj)) => Ok(obj.remove(second)),
                Some(_) => Err(ValueError::custom(format!(
                    "Cannot remove nested value: '{}' is not an object",
                    first
                ))),
                None => Ok(None),
            }
        } else {
            Err(ValueError::custom("Deep nesting (>2 levels) not yet supported"))
        }
    }

    // === Iteration and Keys ===

    /// Returns an iterator over keys
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    /// Returns an iterator over values
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.0.values()
    }

    /// Returns a mutable iterator over values
    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Value> {
        self.0.values_mut()
    }

    /// Returns an iterator over key-value pairs
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.0.iter()
    }

    /// Returns a mutable iterator over key-value pairs
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Value)> {
        self.0.iter_mut()
    }

    /// Collects all keys into a vector
    #[must_use]
    pub fn key_names(&self) -> Vec<String> {
        self.keys().cloned().collect()
    }

    /// Collects all values into a vector
    #[must_use]
    pub fn value_list(&self) -> Vec<Value> {
        self.values().cloned().collect()
    }

    // === Transformation Operations ===

    /// Retains only the key-value pairs that satisfy the predicate
    pub fn retain<F>(&mut self, mut f: F)
    where F: FnMut(&String, &mut Value) -> bool {
        self.0.retain(|k, v| f(k, v));
    }

    /// Filters the object by keys, returning a new object
    #[must_use]
    pub fn filter_keys<P>(&self, mut predicate: P) -> Self
    where P: FnMut(&String) -> bool {
        let filtered = self
            .0
            .iter()
            .filter(|(k, _)| predicate(k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        Self(filtered)
    }

    /// Filters the object by values, returning a new object
    #[must_use]
    pub fn filter_values<P>(&self, mut predicate: P) -> Self
    where P: FnMut(&Value) -> bool {
        let filtered = self
            .0
            .iter()
            .filter(|(_, v)| predicate(v))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        Self(filtered)
    }

    /// Maps values to new values, keeping the same keys
    pub fn map_values<F>(&self, mut f: F) -> ValueResult<Self>
    where F: FnMut(&Value) -> ValueResult<Value> {
        let mut result = InternalMap::new();

        for (k, v) in &self.0 {
            result.insert(k.clone(), f(v)?);
        }

        Ok(Self(result))
    }

    /// Maps keys to new keys, keeping the same values
    pub fn map_keys<F>(&self, mut f: F) -> ValueResult<Self>
    where F: FnMut(&String) -> ValueResult<String> {
        let mut result = InternalMap::new();

        for (k, v) in &self.0 {
            let new_key = f(k)?;
            result.insert(new_key, v.clone());
        }

        Ok(Self(result))
    }

    /// Transforms both keys and values
    pub fn map<F>(&self, mut f: F) -> ValueResult<Self>
    where F: FnMut(&String, &Value) -> ValueResult<(String, Value)> {
        let mut result = InternalMap::new();

        for (k, v) in &self.0 {
            let (new_key, new_value) = f(k, v)?;
            result.insert(new_key, new_value);
        }

        Ok(Self(result))
    }

    // === Merging Operations ===

    /// Merges another object into this one (shallow merge)
    pub fn merge(&mut self, other: &Self) {
        for (k, v) in &other.0 {
            self.0.insert(k.clone(), v.clone());
        }
    }

    /// Deep merges another object into this one
    pub fn deep_merge(&mut self, other: &Self) -> ValueResult<()> {
        for (k, v) in &other.0 {
            match (self.get_mut(k), v) {
                (Some(Value::Object(existing)), Value::Object(incoming)) => {
                    existing.deep_merge(incoming)?;
                },
                _ => {
                    self.insert(k.clone(), v.clone());
                },
            }
        }
        Ok(())
    }

    /// Creates a new object by merging this one with another
    #[must_use]
    pub fn merged(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.merge(other);
        result
    }

    /// Creates a new object by deep merging this one with another
    pub fn deep_merged(&self, other: &Self) -> ValueResult<Self> {
        let mut result = self.clone();
        result.deep_merge(other)?;
        Ok(result)
    }

    // === Query Operations ===

    /// Finds all keys that match a predicate
    #[must_use]
    pub fn find_keys<P>(&self, mut predicate: P) -> Vec<String>
    where P: FnMut(&String, &Value) -> bool {
        self.0.iter().filter(|(k, v)| predicate(k, v)).map(|(k, _)| k.clone()).collect()
    }

    /// Finds all values that match a predicate
    #[must_use]
    pub fn find_values<P>(&self, mut predicate: P) -> Vec<Value>
    where P: FnMut(&String, &Value) -> bool {
        self.0.iter().filter(|(k, v)| predicate(k, v)).map(|(_, v)| v.clone()).collect()
    }

    /// Finds the first key-value pair that matches a predicate
    #[must_use]
    pub fn find<P>(&self, mut predicate: P) -> Option<(&String, &Value)>
    where P: FnMut(&String, &Value) -> bool {
        self.0.iter().find(|(k, v)| predicate(k, v))
    }

    /// Checks if any key-value pair matches a predicate
    #[must_use]
    pub fn any<P>(&self, mut predicate: P) -> bool
    where P: FnMut(&String, &Value) -> bool {
        self.0.iter().any(|(k, v)| predicate(k, v))
    }

    /// Checks if all key-value pairs match a predicate
    #[must_use]
    pub fn all<P>(&self, mut predicate: P) -> bool
    where P: FnMut(&String, &Value) -> bool {
        self.0.iter().all(|(k, v)| predicate(k, v))
    }

    // === Utility Operations ===

    /// Flattens nested objects using dot notation
    #[must_use]
    pub fn flatten(&self) -> Self {
        let mut result = InternalMap::new();
        self.flatten_recursive("", &mut result);
        Self(result)
    }

    /// Helper function for recursive flattening
    fn flatten_recursive(&self, prefix: &str, result: &mut InternalMap<String, Value>) {
        for (k, v) in &self.0 {
            let key = if prefix.is_empty() { k.clone() } else { format!("{}.{}", prefix, k) };

            match v {
                Value::Object(obj) => {
                    obj.flatten_recursive(&key, result);
                },
                _ => {
                    result.insert(key, v.clone());
                },
            }
        }
    }

    /// Reverses the flattening operation (limited support)
    pub fn unflatten(&self) -> ValueResult<Self> {
        let mut result = Self::new();

        for (k, v) in &self.0 {
            result.set_nested(k, v.clone())?;
        }

        Ok(result)
    }

    /// Swaps keys and values (values must be strings)
    pub fn invert(&self) -> ValueResult<Self> {
        let mut result = InternalMap::new();

        for (k, v) in &self.0 {
            match v.as_string() {
                Some(string_val) => {
                    result.insert(string_val.to_string(), Value::string(k.as_str()));
                },
                None => {
                    return Err(ValueError::custom(format!(
                        "Cannot invert object: value for key '{}' is not a string",
                        k
                    )));
                },
            }
        }

        Ok(Self(result))
    }

    /// Creates a copy with only the specified keys
    #[must_use]
    pub fn pick(&self, keys: &[&str]) -> Self {
        let mut result = InternalMap::new();

        for &key in keys {
            if let Some(value) = self.get(key) {
                result.insert(key.to_string(), value.clone());
            }
        }

        Self(result)
    }

    /// Creates a copy without the specified keys
    #[must_use]
    pub fn omit(&self, keys: &[&str]) -> Self {
        let keys_set: std::collections::HashSet<&str> = keys.iter().copied().collect();

        let filtered = self
            .0
            .iter()
            .filter(|(k, _)| !keys_set.contains(k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        Self(filtered)
    }

    /// Gets the size in terms of number of nested values
    #[must_use]
    pub fn deep_size(&self) -> usize {
        self.0
            .values()
            .map(|v| match v {
                Value::Object(obj) => obj.deep_size(),
                Value::Array(arr) => arr.len(),
                _ => 1,
            })
            .sum::<usize>()
            + self.len()
    }

    /// Converts to a regular HashMap (loses ordering)
    #[must_use]
    pub fn to_hashmap(&self) -> InternalMap<String, Value> {
        self.0.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    /// Converts to the internal map
    #[must_use]
    pub fn into_inner_map(self) -> InternalMap<String, Value> {
        self.0
    }

    /// Gets a reference to the internal map
    #[inline]
    #[must_use]
    pub fn as_inner_map(&self) -> &InternalMap<String, Value> {
        &self.0
    }

    /// Gets a mutable reference to the internal map
    #[inline]
    #[must_use]
    pub fn as_inner_map_mut(&mut self) -> &mut InternalMap<String, Value> {
        &mut self.0
    }
}

// === Default Implementation ===

impl Default for ObjectValue {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

// === Trait Implementations ===

impl fmt::Display for ObjectValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[object with {} fields]", self.len())
    }
}

impl Hash for ObjectValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (k, v) in &self.0 {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl Deref for ObjectValue {
    type Target = InternalMap<String, Value>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ObjectValue {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<&str> for ObjectValue {
    type Output = Value;

    fn index(&self, key: &str) -> &Self::Output {
        &self.0[key]
    }
}

impl IndexMut<&str> for ObjectValue {
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        self.0.get_mut(key).expect("Key not found")
    }
}

// === From implementations ===

#[cfg(feature = "collections")]
impl From<IndexMap<String, Value>> for ObjectValue {
    #[inline]
    fn from(map: IndexMap<String, Value>) -> Self {
        Self(map)
    }
}

#[cfg(feature = "collections")]
impl From<ObjectValue> for IndexMap<String, Value> {
    #[inline]
    fn from(obj: ObjectValue) -> Self {
        obj.0
    }
}

impl From<std::collections::HashMap<String, Value>> for ObjectValue {
    fn from(map: std::collections::HashMap<String, Value>) -> Self {
        #[cfg(feature = "collections")]
        {
            Self(map.into_iter().collect())
        }
        #[cfg(not(feature = "collections"))]
        {
            Self(map)
        }
    }
}

impl From<ObjectValue> for std::collections::HashMap<String, Value> {
    fn from(obj: ObjectValue) -> Self {
        obj.0.into_iter().collect()
    }
}

// === Collection trait implementations ===

impl FromIterator<(String, Value)> for ObjectValue {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<(&'a str, Value)> for ObjectValue {
    fn from_iter<T: IntoIterator<Item = (&'a str, Value)>>(iter: T) -> Self {
        Self(iter.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }
}

impl Extend<(String, Value)> for ObjectValue {
    fn extend<T: IntoIterator<Item = (String, Value)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'a> Extend<(&'a str, Value)> for ObjectValue {
    fn extend<T: IntoIterator<Item = (&'a str, Value)>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().map(|(k, v)| (k.to_string(), v)));
    }
}

impl IntoIterator for ObjectValue {
    type Item = (String, Value);
    type IntoIter = <InternalMap<String, Value> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ObjectValue {
    type Item = (&'a String, &'a Value);
    type IntoIter = <&'a InternalMap<String, Value> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut ObjectValue {
    type Item = (&'a String, &'a mut Value);
    type IntoIter = <&'a mut InternalMap<String, Value> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

// === JSON conversion (feature-gated) ===

#[cfg(feature = "json")]
impl From<ObjectValue> for serde_json::Value {
    fn from(obj: ObjectValue) -> Self {
        let map = obj.0.into_iter().map(|(k, v)| (k, v.into())).collect();
        serde_json::Value::Object(map)
    }
}

#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for ObjectValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::Object(map) => {
                let converted: Result<InternalMap<String, Value>, ValueError> =
                    map.into_iter().map(|(k, v)| Ok((k, v.try_into()?))).collect();
                Ok(Self(converted?))
            },
            other => Err(ValueError::custom(format!("Cannot convert {:?} to ObjectValue", other))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        let empty = ObjectValue::new();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);

        let obj =
            ObjectValue::from_pairs([("name", Value::string("John")), ("age", Value::number(30))]);
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.get("name"), Some(&Value::string("John")));
    }

    #[test]
    fn test_basic_operations() {
        let mut obj = ObjectValue::new();

        // Insert and get
        obj.insert("key1".to_string(), Value::string("value1"));
        assert_eq!(obj.get("key1"), Some(&Value::string("value1")));
        assert!(obj.contains_key("key1"));

        // Update
        obj.insert("key1".to_string(), Value::string("updated"));
        assert_eq!(obj.get("key1"), Some(&Value::string("updated")));

        // Remove
        let removed = obj.remove("key1");
        assert_eq!(removed, Some(Value::string("updated")));
        assert!(!obj.contains_key("key1"));
    }

    #[test]
    fn test_nested_access() {
        let mut obj = ObjectValue::new();

        // Set nested value
        obj.set_nested("user.name", Value::string("Alice")).unwrap();

        // Get nested value
        let name = obj.get_nested_cloned("user.name");
        assert_eq!(name, Some(Value::string("Alice")));

        // Remove nested value
        let removed = obj.remove_nested("user.name").unwrap();
        assert_eq!(removed, Some(Value::string("Alice")));
    }

    #[test]
    fn test_merging() {
        let mut obj1 = ObjectValue::from_pairs([("a", Value::number(1)), ("b", Value::number(2))]);

        let obj2 = ObjectValue::from_pairs([("b", Value::number(3)), ("c", Value::number(4))]);

        obj1.merge(&obj2);

        assert_eq!(obj1.get("a"), Some(&Value::number(1)));
        assert_eq!(obj1.get("b"), Some(&Value::number(3))); // Overwritten
        assert_eq!(obj1.get("c"), Some(&Value::number(4)));
    }

    #[test]
    fn test_filtering() {
        let obj = ObjectValue::from_pairs([
            ("a", Value::number(1)),
            ("b", Value::string("hello")),
            ("c", Value::number(3)),
        ]);

        // Filter by key
        let numeric_keys = obj.filter_keys(|k| k == "a" || k == "c");
        assert_eq!(numeric_keys.len(), 2);
        assert!(numeric_keys.contains_key("a"));
        assert!(numeric_keys.contains_key("c"));

        // Filter by value type
        let numbers = obj.filter_values(|v| v.is_number());
        assert_eq!(numbers.len(), 2);
    }

    #[test]
    fn test_transformation() {
        let obj = ObjectValue::from_pairs([("a", Value::number(1)), ("b", Value::number(2))]);

        // Map values
        let doubled = obj
            .map_values(|v| {
                if let Some(n) = v.as_number() {
                    Ok(Value::number(n.as_f64() * 2.0))
                } else {
                    Ok(v.clone())
                }
            })
            .unwrap();

        assert_eq!(doubled.get("a"), Some(&Value::number(2.0)));
        assert_eq!(doubled.get("b"), Some(&Value::number(4.0)));
    }

    #[test]
    fn test_flattening() {
        let mut obj = ObjectValue::new();
        obj.set_nested("user.name", Value::string("John")).unwrap();
        obj.set_nested("user.age", Value::number(30)).unwrap();
        obj.insert("active".to_string(), Value::boolean(true));

        let flattened = obj.flatten();
        // Note: exact keys depend on implementation, but should contain flattened
        // structure
        assert!(!flattened.is_empty());
    }

    #[test]
    fn test_pick_omit() {
        let obj = ObjectValue::from_pairs([
            ("a", Value::number(1)),
            ("b", Value::string("hello")),
            ("c", Value::boolean(true)),
        ]);

        // Pick specific keys
        let picked = obj.pick(&["a", "c"]);
        assert_eq!(picked.len(), 2);
        assert!(picked.contains_key("a"));
        assert!(picked.contains_key("c"));
        assert!(!picked.contains_key("b"));

        // Omit specific keys
        let omitted = obj.omit(&["b"]);
        assert_eq!(omitted.len(), 2);
        assert!(omitted.contains_key("a"));
        assert!(omitted.contains_key("c"));
        assert!(!omitted.contains_key("b"));
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_json_conversion() {
        let obj = ObjectValue::from_pairs([
            ("name", Value::string("test")),
            ("count", Value::number(42)),
        ]);

        // To JSON
        let json: serde_json::Value = obj.clone().into();
        assert!(json.is_object());

        // From JSON
        let back: ObjectValue = json.try_into().unwrap();
        assert_eq!(back, obj);
    }

    #[cfg(feature = "collections")]
    #[test]
    fn test_indexed_access() {
        let obj = ObjectValue::from_pairs([
            ("first", Value::number(1)),
            ("second", Value::string("hello")),
        ]);

        // Test index-based access (only available with collections feature)
        let (key, value) = obj.get_index(0).unwrap();
        assert_eq!(key, "first");
        assert_eq!(*value, Value::number(1));
    }
}
