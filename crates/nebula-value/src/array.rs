use std::fmt;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[cfg(feature = "collections")]
use indexmap::IndexSet;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Value, ValueError, ValueResult};

/// Array value type with efficient operations and functional programming
/// support
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ArrayValue(Vec<Value>);

impl ArrayValue {
    /// Creates a new array value from a vector
    #[inline]
    #[must_use]
    pub fn new(values: impl Into<Vec<Value>>) -> Self {
        Self(values.into())
    }

    /// Creates a new array value from a Vec<Value> directly (const-friendly)
    #[inline]
    #[must_use]
    pub const fn from_vec(values: Vec<Value>) -> Self {
        Self(values)
    }

    /// Creates an empty array
    #[inline]
    #[must_use]
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    /// Creates an array with specified capacity
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Creates an array filled with n copies of the given value
    #[must_use]
    pub fn filled(value: &Value, count: usize) -> Self {
        Self(vec![value.clone(); count])
    }

    /// Creates an array from a range of numbers
    pub fn range(start: i64, end: i64, step: Option<i64>) -> ValueResult<Self> {
        let step = step.unwrap_or(1);
        if step == 0 {
            return Err(ValueError::custom("Step cannot be zero in range"));
        }

        let mut values = Vec::new();
        let mut current = start;

        if step > 0 {
            while current < end {
                values.push(Value::number(current));
                current += step;
            }
        } else {
            while current > end {
                values.push(Value::number(current));
                current += step;
            }
        }

        Ok(Self(values))
    }

    // === Basic Operations ===

    /// Returns the length of the array
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the array is empty
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the capacity of the array
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
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

    /// Clears all elements from the array
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    // === Element Access ===

    /// Safe element access by index
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.0.get(index)
    }

    /// Safe mutable element access by index
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.0.get_mut(index)
    }

    /// Safe element access with error handling
    #[inline]
    pub fn try_get(&self, index: usize) -> ValueResult<&Value> {
        self.get(index).ok_or_else(|| ValueError::index_out_of_bounds(index, self.len()))
    }

    /// Safe mutable element access with error handling
    #[inline]
    pub fn try_get_mut(&mut self, index: usize) -> ValueResult<&mut Value> {
        let len = self.len();
        self.get_mut(index).ok_or_else(|| ValueError::index_out_of_bounds(index, len))
    }

    /// Gets the first element
    #[inline]
    #[must_use]
    pub fn first(&self) -> Option<&Value> {
        self.0.first()
    }

    /// Gets the last element
    #[inline]
    #[must_use]
    pub fn last(&self) -> Option<&Value> {
        self.0.last()
    }

    /// Gets the first element mutably
    #[inline]
    #[must_use]
    pub fn first_mut(&mut self) -> Option<&mut Value> {
        self.0.first_mut()
    }

    /// Gets the last element mutably
    #[inline]
    #[must_use]
    pub fn last_mut(&mut self) -> Option<&mut Value> {
        self.0.last_mut()
    }

    // === Modification Operations ===

    /// Appends an element to the back of the array
    #[inline]
    pub fn push(&mut self, value: Value) {
        self.0.push(value);
    }

    /// Removes and returns the last element
    #[inline]
    #[must_use]
    pub fn pop(&mut self) -> Option<Value> {
        self.0.pop()
    }

    /// Inserts an element at position index
    #[inline]
    pub fn insert(&mut self, index: usize, element: Value) -> ValueResult<()> {
        if index > self.len() {
            return Err(ValueError::index_out_of_bounds(index, self.len()));
        }
        self.0.insert(index, element);
        Ok(())
    }

    /// Removes and returns the element at position index
    #[inline]
    pub fn remove(&mut self, index: usize) -> ValueResult<Value> {
        if index >= self.len() {
            return Err(ValueError::index_out_of_bounds(index, self.len()));
        }
        Ok(self.0.remove(index))
    }

    /// Removes an element and returns it, or None if not found
    #[must_use]
    pub fn remove_item(&mut self, item: &Value) -> Option<Value> {
        if let Some(pos) = self.0.iter().position(|x| x == item) {
            Some(self.0.remove(pos))
        } else {
            None
        }
    }

    /// Retains only the elements specified by the predicate
    pub fn retain<F>(&mut self, mut f: F)
    where F: FnMut(&Value) -> bool {
        self.0.retain(|value| f(value));
    }

    /// Removes consecutive repeated elements
    pub fn dedup(&mut self) {
        self.0.dedup();
    }

    /// Removes consecutive repeated elements using a custom comparison
    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where F: FnMut(&mut Value, &mut Value) -> bool {
        self.0.dedup_by(same_bucket);
    }

    // === Slicing and Subranges ===

    /// Returns a slice of the array from start to end (exclusive)
    pub fn slice(&self, start: usize, end: usize) -> ValueResult<ArrayValue> {
        if start > end {
            return Err(ValueError::custom(format!(
                "Invalid slice range: start ({}) > end ({})",
                start, end
            )));
        }
        if end > self.len() {
            return Err(ValueError::index_out_of_bounds(end, self.len()));
        }
        Ok(ArrayValue::new(self.0[start..end].to_vec()))
    }

    /// Returns the first n elements
    #[must_use]
    pub fn take(&self, n: usize) -> ArrayValue {
        let end = n.min(self.len());
        ArrayValue::new(self.0[..end].to_vec())
    }

    /// Returns all elements after skipping n
    #[must_use]
    pub fn skip(&self, n: usize) -> ArrayValue {
        let start = n.min(self.len());
        ArrayValue::new(self.0[start..].to_vec())
    }

    /// Returns elements from start index
    #[inline]
    #[must_use]
    pub fn drop(&self, n: usize) -> ArrayValue {
        self.skip(n)
    }

    /// Returns elements up to but not including the nth element
    #[must_use]
    pub fn take_while<P>(&self, mut predicate: P) -> ArrayValue
    where P: FnMut(&Value) -> bool {
        let mut result = Vec::new();
        for value in &self.0 {
            if predicate(value) {
                result.push(value.clone());
            } else {
                break;
            }
        }
        ArrayValue::new(result)
    }

    /// Skips elements while predicate is true, returns the rest
    #[must_use]
    pub fn skip_while<P>(&self, mut predicate: P) -> ArrayValue
    where P: FnMut(&Value) -> bool {
        let mut found_false = false;
        let result: Vec<Value> = self
            .0
            .iter()
            .filter_map(|value| {
                if !found_false && predicate(value) {
                    None
                } else {
                    found_false = true;
                    Some(value.clone())
                }
            })
            .collect();
        ArrayValue::new(result)
    }

    // === Functional Operations ===

    /// Applies a function to each element and returns a new array
    pub fn map<F>(&self, mut f: F) -> ValueResult<ArrayValue>
    where F: FnMut(&Value) -> ValueResult<Value> {
        let mut result = Vec::with_capacity(self.len());
        for value in &self.0 {
            result.push(f(value)?);
        }
        Ok(ArrayValue::new(result))
    }

    /// Applies a fallible function and filters out errors
    #[must_use]
    pub fn filter_map<F>(&self, f: F) -> ArrayValue
    where F: FnMut(&Value) -> Option<Value> {
        let result: Vec<Value> = self.0.iter().filter_map(f).collect();
        ArrayValue::new(result)
    }

    /// Filters elements based on a predicate
    #[must_use]
    pub fn filter<P>(&self, mut predicate: P) -> ArrayValue
    where P: FnMut(&Value) -> bool {
        let result: Vec<Value> = self.0.iter().filter(|value| predicate(value)).cloned().collect();
        ArrayValue::new(result)
    }

    /// Finds the first element matching a predicate
    #[must_use]
    pub fn find<P>(&self, mut predicate: P) -> Option<&Value>
    where P: FnMut(&Value) -> bool {
        self.0.iter().find(|value| predicate(value))
    }

    /// Finds the index of the first element matching a predicate
    #[must_use]
    pub fn find_index<P>(&self, predicate: P) -> Option<usize>
    where P: FnMut(&Value) -> bool {
        self.0.iter().position(predicate)
    }

    /// Returns true if any element matches the predicate
    #[must_use]
    pub fn any<P>(&self, predicate: P) -> bool
    where P: FnMut(&Value) -> bool {
        self.0.iter().any(predicate)
    }

    /// Returns true if all elements match the predicate
    #[must_use]
    pub fn all<P>(&self, predicate: P) -> bool
    where P: FnMut(&Value) -> bool {
        self.0.iter().all(predicate)
    }

    /// Reduces the array to a single value using an accumulator function
    pub fn reduce<F>(&self, mut f: F) -> ValueResult<Option<Value>>
    where F: FnMut(Value, &Value) -> ValueResult<Value> {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            let mut acc = first.clone();
            for value in iter {
                acc = f(acc, value)?;
            }
            Ok(Some(acc))
        } else {
            Ok(None)
        }
    }

    /// Folds the array with an initial value
    pub fn fold<T, F>(&self, init: T, mut f: F) -> ValueResult<T>
    where F: FnMut(T, &Value) -> ValueResult<T> {
        let mut acc = init;
        for value in &self.0 {
            acc = f(acc, value)?;
        }
        Ok(acc)
    }

    // === Search and Contains ===

    /// Checks if the array contains a value
    #[inline]
    #[must_use]
    pub fn contains(&self, needle: &Value) -> bool {
        self.0.contains(needle)
    }

    /// Finds the index of a value
    #[inline]
    #[must_use]
    pub fn index_of(&self, needle: &Value) -> Option<usize> {
        self.0.iter().position(|value| value == needle)
    }

    /// Finds the last index of a value
    #[inline]
    #[must_use]
    pub fn last_index_of(&self, needle: &Value) -> Option<usize> {
        self.0.iter().rposition(|value| value == needle)
    }

    /// Counts occurrences of a value
    #[must_use]
    pub fn count_value(&self, needle: &Value) -> usize {
        self.0.iter().filter(|value| *value == needle).count()
    }

    // === Sorting and Ordering ===

    /// Sorts the array in-place
    pub fn sort(&mut self) -> ValueResult<()> {
        self.0.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        Ok(())
    }

    /// Returns a sorted copy of the array
    pub fn sorted(&self) -> ValueResult<ArrayValue> {
        let mut result = self.clone();
        result.sort()?;
        Ok(result)
    }

    /// Sorts the array by a key function
    pub fn sort_by<F, K>(&mut self, mut f: F) -> ValueResult<()>
    where
        F: FnMut(&Value) -> ValueResult<K>,
        K: Ord,
    {
        // Pre-compute all keys to avoid repeated evaluation
        let mut indexed: Vec<(usize, K)> = Vec::with_capacity(self.len());
        for (i, value) in self.0.iter().enumerate() {
            indexed.push((i, f(value)?));
        }

        // Sort by key
        indexed.sort_by(|a, b| a.1.cmp(&b.1));

        // Reorder original vector
        let mut sorted = Vec::with_capacity(self.len());
        for (original_index, _) in indexed {
            sorted.push(self.0[original_index].clone());
        }

        self.0 = sorted;
        Ok(())
    }

    /// Reverses the array in-place
    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    /// Returns a reversed copy of the array
    #[must_use]
    pub fn reversed(&self) -> ArrayValue {
        let mut result = self.clone();
        result.reverse();
        result
    }

    // === Array Combination ===

    /// Concatenates with another array
    #[must_use]
    pub fn concat(&self, other: &ArrayValue) -> ArrayValue {
        let mut result = self.0.clone();
        result.extend_from_slice(&other.0);
        ArrayValue::new(result)
    }

    /// Appends another array to this one
    #[inline]
    pub fn extend(&mut self, other: &ArrayValue) {
        self.0.extend_from_slice(&other.0);
    }

    /// Joins array elements into a string
    pub fn join(&self, separator: &str) -> ValueResult<String> {
        let strings: Result<Vec<String>, ValueError> = self
            .0
            .iter()
            .map(|value| match value {
                Value::String(s) => Ok(s.to_string()),
                other => Ok(other.to_string()),
            })
            .collect();

        Ok(strings?.join(separator))
    }

    /// Flattens nested arrays by one level
    #[must_use]
    pub fn flatten(&self) -> ArrayValue {
        let mut result = Vec::new();
        for value in &self.0 {
            if let Some(arr) = value.as_array() {
                result.extend_from_slice(&arr.0);
            } else {
                result.push(value.clone());
            }
        }
        ArrayValue::new(result)
    }

    /// Flattens deeply nested arrays
    #[must_use]
    pub fn flatten_deep(&self) -> ArrayValue {
        let mut result = Vec::new();
        for value in &self.0 {
            if let Some(arr) = value.as_array() {
                result.extend_from_slice(&arr.flatten_deep().0);
            } else {
                result.push(value.clone());
            }
        }
        ArrayValue::new(result)
    }

    // === Utility Methods ===

    /// Creates chunks of specified size
    pub fn chunks(&self, chunk_size: usize) -> ValueResult<Vec<ArrayValue>> {
        if chunk_size == 0 {
            return Err(ValueError::custom("Chunk size cannot be zero"));
        }

        let chunks: Vec<ArrayValue> =
            self.0.chunks(chunk_size).map(|chunk| ArrayValue::new(chunk.to_vec())).collect();

        Ok(chunks)
    }

    /// Removes duplicate values (preserving order when collections feature is
    /// enabled)
    #[must_use]
    pub fn unique(&self) -> ArrayValue {
        #[cfg(feature = "collections")]
        {
            let mut seen = IndexSet::new();
            let mut result = Vec::new();

            for value in &self.0 {
                if seen.insert(value) {
                    result.push(value.clone());
                }
            }

            ArrayValue::new(result)
        }

        #[cfg(not(feature = "collections"))]
        {
            let mut seen = std::collections::HashSet::<String>::new();
            let mut result = Vec::new();

            for value in &self.0 {
                let key = value.to_string();
                if seen.insert(key) {
                    result.push(value.clone());
                }
            }

            ArrayValue::new(result)
        }
    }

    /// Converts to Vec<Value> consuming self
    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<Value> {
        self.0
    }

    /// Returns an iterator over the values
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the values
    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Value> {
        self.0.iter_mut()
    }
}

// === Trait Implementations ===

impl Default for ArrayValue {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl Deref for ArrayValue {
    type Target = Vec<Value>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ArrayValue {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<usize> for ArrayValue {
    type Output = Value;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for ArrayValue {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl fmt::Display for ArrayValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[array with {} items]", self.len())
    }
}

// From implementations
impl<T> From<Vec<T>> for ArrayValue
where T: Into<Value>
{
    fn from(values: Vec<T>) -> Self {
        ArrayValue::new(values.into_iter().map(Into::into).collect::<Vec<Value>>())
    }
}

impl From<ArrayValue> for Vec<Value> {
    fn from(array: ArrayValue) -> Self {
        array.0
    }
}

// Collection traits
impl FromIterator<Value> for ArrayValue {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        ArrayValue::new(iter.into_iter().collect::<Vec<Value>>())
    }
}

impl Extend<Value> for ArrayValue {
    fn extend<T: IntoIterator<Item = Value>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl IntoIterator for ArrayValue {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ArrayValue {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut ArrayValue {
    type Item = &'a mut Value;
    type IntoIter = std::slice::IterMut<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

// JSON conversion (feature-gated)
#[cfg(feature = "json")]
impl From<ArrayValue> for serde_json::Value {
    fn from(array: ArrayValue) -> Self {
        serde_json::Value::Array(array.0.into_iter().map(|value| value.into()).collect())
    }
}

#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for ArrayValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::Array(arr) => {
                let values: Result<Vec<Value>, ValueError> =
                    arr.into_iter().map(|v| v.try_into()).collect();
                Ok(ArrayValue::new(values?))
            },
            other => Err(ValueError::type_conversion_with_value(
                other.to_string(),
                "ArrayValue",
                format!("{:?}", other),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let arr = ArrayValue::new(vec![Value::string("hello"), Value::number(42)]);
        assert_eq!(arr.len(), 2);
        assert!(!arr.is_empty());
    }

    #[test]
    fn test_array_range() {
        let arr = ArrayValue::range(0, 5, None).unwrap();
        assert_eq!(arr.len(), 5);
        assert_eq!(arr[0], Value::number(0));
        assert_eq!(arr[4], Value::number(4));
    }

    #[test]
    fn test_array_operations() {
        let mut arr = ArrayValue::new(vec![Value::number(1), Value::number(2), Value::number(3)]);

        arr.push(Value::number(4));
        assert_eq!(arr.len(), 4);

        let popped = arr.pop();
        assert_eq!(popped, Some(Value::number(4)));
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_array_functional() {
        let arr = ArrayValue::new(vec![Value::number(1), Value::number(2), Value::number(3)]);

        let doubled = arr
            .map(|v| {
                if let Some(n) = v.as_number() {
                    Ok(Value::number(n.as_i64().unwrap() * 2))
                } else {
                    Ok(v.clone())
                }
            })
            .unwrap();

        assert_eq!(doubled[0], Value::number(2));
        assert_eq!(doubled[1], Value::number(4));
        assert_eq!(doubled[2], Value::number(6));
    }

    #[test]
    fn test_array_unique() {
        let arr = ArrayValue::new(vec![
            Value::number(1),
            Value::number(2),
            Value::number(1),
            Value::number(3),
            Value::number(2),
        ]);

        let unique = arr.unique();
        assert_eq!(unique.len(), 3);
        assert!(unique.contains(&Value::number(1)));
        assert!(unique.contains(&Value::number(2)));
        assert!(unique.contains(&Value::number(3)));
    }

    #[cfg(all(feature = "json", feature = "serde"))]
    #[test]
    fn test_json_conversion() {
        let arr = ArrayValue::new(vec![Value::string("hello"), Value::number(42)]);
        let json: serde_json::Value = arr.clone().into();

        match json {
            serde_json::Value::Array(ref json_arr) => {
                assert_eq!(json_arr.len(), 2);
            },
            _ => panic!("Expected JSON array"),
        }

        let back: ArrayValue = json.try_into().unwrap();
        assert_eq!(back, arr);
    }
}
