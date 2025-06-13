use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{BinaryValue, FileValue, StringValue, ValueError, ValueResult};

/// Value structure automatically determines ultra-simplified mode value - type
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModeValue {
    /// Parameter key/name
    pub key: String,
    /// Value - type is auto-detected (String = text/list, Object = file)
    pub value: ModeTypeValue,
}

/// Value types - auto-discriminated during serialization/deserialization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))] // This makes serde auto-detect the variant!
pub enum ModeTypeValue {
    /// String value (auto-detected when JSON value is string)
    String(StringValue),
    /// File value (auto-detected when JSON value is object)
    File(Box<FileValue>),
}

impl ModeValue {
    // === Construction ===

    /// Creates a new mode value with key and value
    #[must_use]
    pub fn new(key: impl Into<String>, value: ModeTypeValue) -> Self {
        Self { key: key.into(), value }
    }

    /// Creates a text mode with string value
    #[must_use]
    pub fn text(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(key, ModeTypeValue::string(value.into()))
    }

    /// Creates a list mode with selected value (same as text - context
    /// determines usage)
    #[must_use]
    pub fn list(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::text(key, value) // Functionally identical to text
    }

    /// Creates a file mode with FileValue
    #[must_use]
    pub fn file(key: impl Into<String>, value: FileValue) -> Self {
        Self::new(key, ModeTypeValue::file(value))
    }

    /// Creates a file mode with binary data (stored as InMemory file)
    #[must_use]
    pub fn file_binary(
        key: impl Into<String>,
        value: BinaryValue,
        filename: Option<String>,
    ) -> Self {
        Self::file(key, FileValue::from_binary(value, filename))
    }

    /// Creates a file mode with URL string (URL stored as string in file)
    #[must_use]
    pub fn file_url(key: impl Into<String>, url: impl Into<String>) -> Self {
        Self::file(key, FileValue::from_url(url.into(), Default::default()))
    }

    /// Creates a file mode from uploaded bytes
    #[must_use]
    pub fn file_upload(key: impl Into<String>, data: Vec<u8>, filename: Option<String>) -> Self {
        Self::file(key, FileValue::from_bytes(data, filename))
    }

    /// Creates a file mode from remote storage
    #[must_use]
    pub fn file_remote(
        key: impl Into<String>,
        storage_key: String,
        storage_type: crate::file::StorageType,
        metadata: crate::file::FileMetadata,
    ) -> Self {
        Self::file(key, FileValue::from_remote(storage_key, storage_type, metadata))
    }

    /// Creates a file mode from a generated file
    #[cfg(feature = "json")]
    #[must_use]
    pub fn file_generated(
        key: impl Into<String>,
        generator_id: String,
        parameters: serde_json::Value,
        metadata: crate::file::FileMetadata,
    ) -> Self {
        Self::file(key, FileValue::from_generator(generator_id, parameters, metadata))
    }

    /// Creates a file mode from a generated file (without JSON support)
    #[cfg(not(feature = "json"))]
    #[must_use]
    pub fn file_generated(
        key: impl Into<String>,
        generator_id: String,
        parameters: String,
        metadata: crate::file::FileMetadata,
    ) -> Self {
        Self::file(key, FileValue::from_generator(generator_id, parameters, metadata))
    }

    // === Accessors ===

    /// Get the parameter key
    #[inline]
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the current value
    #[inline]
    #[must_use]
    pub fn value(&self) -> &ModeTypeValue {
        &self.value
    }

    /// Get mutable value reference
    #[inline]
    #[must_use]
    pub fn value_mut(&mut self) -> &mut ModeTypeValue {
        &mut self.value
    }

    // === Type detection (inferred from value) ===

    /// Get the inferred mode type based on value
    #[must_use]
    pub fn inferred_mode(&self) -> ModeType {
        match &self.value {
            ModeTypeValue::String(_) => ModeType::Text, // Could be Text or List, context
            // determines
            ModeTypeValue::File(_) => ModeType::File,
        }
    }

    /// Check if this is a text/list parameter (string value)
    #[must_use]
    pub fn is_text_or_list(&self) -> bool {
        matches!(self.value, ModeTypeValue::String(_))
    }

    /// Check if this is a file parameter
    #[must_use]
    pub fn is_file(&self) -> bool {
        matches!(self.value, ModeTypeValue::File(_))
    }

    // === Type-specific accessors ===

    /// Get value as string (if possible)
    #[must_use]
    pub fn as_string(&self) -> Option<&str> {
        self.value.as_string()
    }

    /// Get value as a file (if possible)
    #[must_use]
    pub fn as_file(&self) -> Option<&FileValue> {
        self.value.as_file()
    }

    /// Get mutable file reference (if possible)
    #[must_use]
    pub fn as_file_mut(&mut self) -> Option<&mut FileValue> {
        self.value.as_file_mut()
    }

    /// Get as binary data (from a file if InMemory)
    #[must_use]
    pub fn as_binary(&self) -> Option<&BinaryValue> {
        self.value.as_binary()
    }

    // === File-specific helpers ===

    /// Get filename (if file mode)
    #[must_use]
    pub fn filename(&self) -> Option<&str> {
        self.value.filename()
    }

    /// Get MIME type (if file mode)
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        self.value.mime_type()
    }

    /// Check if this represents an image file
    #[must_use]
    pub fn is_image(&self) -> bool {
        self.value.is_image()
    }

    /// Check if this represents a text file
    #[must_use]
    pub fn is_text(&self) -> bool {
        self.value.is_text()
    }

    // === Mutators ===

    /// Set the key
    pub fn set_key(&mut self, key: impl Into<String>) {
        self.key = key.into();
    }

    /// Set the value
    pub fn set_value(&mut self, value: ModeTypeValue) {
        self.value = value;
    }

    // === Utility Methods ===

    /// Check if the value is empty/null
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Get the size/length of the value
    #[must_use]
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Get detailed description of the current value
    #[must_use]
    pub fn detailed_description(&self) -> String {
        format!("{}:({})", self.key, self.value.detailed_info())
    }

    // === Conversion Methods ===

    /// Clone with a new key
    #[must_use]
    pub fn with_key(&self, new_key: impl Into<String>) -> Self {
        Self { key: new_key.into(), value: self.value.clone() }
    }

    /// Clone with new value
    #[must_use]
    pub fn with_value(&self, new_value: ModeTypeValue) -> Self {
        Self { key: self.key.clone(), value: new_value }
    }

    /// Extract the inner value, consuming self
    #[must_use]
    pub fn into_value(self) -> ModeTypeValue {
        self.value
    }

    /// Extract all parts, consuming self
    #[must_use]
    pub fn into_parts(self) -> (String, ModeTypeValue) {
        (self.key, self.value)
    }

    // === Data access methods ===

    /// Try to get string representation of the value
    pub fn to_string_representation(&self) -> ValueResult<String> {
        self.value.to_string_representation()
    }

    /// Try to get binary data from the value
    pub fn to_binary_data(&self) -> ValueResult<BinaryValue> {
        self.value.to_binary_data()
    }
}

// Keep ModeType enum for backward compatibility and explicit typing when needed
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum ModeType {
    Text,
    List,
    File,
}

impl ModeType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::List => "list",
            Self::File => "file",
        }
    }
}

// === ModeTypeValue implementation ===

impl ModeTypeValue {
    /// Create a string mode value
    #[inline]
    #[must_use]
    pub fn string(value: impl Into<StringValue>) -> Self {
        Self::String(value.into())
    }

    /// Create a file mode value
    #[inline]
    #[must_use]
    pub fn file(value: FileValue) -> Self {
        Self::File(Box::new(value))
    }

    /// Create a file mode value from binary data
    #[inline]
    #[must_use]
    pub fn file_from_binary(binary: BinaryValue, filename: Option<String>) -> Self {
        Self::File(Box::new(FileValue::from_binary(binary, filename)))
    }

    /// Create a file mode value from bytes
    #[inline]
    #[must_use]
    pub fn file_from_bytes(data: Vec<u8>, filename: Option<String>) -> Self {
        Self::File(Box::new(FileValue::from_bytes(data, filename)))
    }

    /// Check if this is a string value
    #[inline]
    #[must_use]
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Check if this is a file value
    #[inline]
    #[must_use]
    pub const fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    /// Get as string if possible
    #[must_use]
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_ref()),
            Self::File(_) => None,
        }
    }

    /// Get as a file if possible
    #[must_use]
    pub fn as_file(&self) -> Option<&FileValue> {
        match self {
            Self::String(_) => None,
            Self::File(f) => Some(f),
        }
    }

    /// Get mutable file reference if possible
    #[must_use]
    pub fn as_file_mut(&mut self) -> Option<&mut FileValue> {
        match self {
            Self::String(_) => None,
            Self::File(f) => Some(f),
        }
    }

    /// Get as binary data (from a file if InMemory)
    #[must_use]
    pub fn as_binary(&self) -> Option<&BinaryValue> {
        match self {
            Self::String(_) => None,
            Self::File(f) => f.binary_data(),
        }
    }

    /// Check if the value is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::String(s) => s.is_empty(),
            Self::File(f) => f.size().unwrap_or(0) == 0,
        }
    }

    /// Get the length/size of the value
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::String(s) => s.len(),
            Self::File(f) => f.size().unwrap_or(0),
        }
    }

    /// Get a human-readable description of the value type
    #[must_use]
    pub fn type_description(&self) -> &'static str {
        match self {
            Self::String(_) => "string",
            Self::File(_) => "file",
        }
    }

    /// Get detailed information about the value
    #[must_use]
    pub fn detailed_info(&self) -> String {
        match self {
            Self::String(s) => format!("string ({} chars)", s.len()),
            Self::File(f) => {
                let size = f
                    .size()
                    .map(|s| format!("{s} bytes"))
                    .unwrap_or_else(|| "unknown size".to_string());
                let filename = f.filename().unwrap_or("unnamed");
                let file_type = f.file_type();
                format!("file '{filename}' ({file_type}, {size})")
            },
        }
    }

    // === File-specific helpers ===

    /// Get filename if this is a file value
    #[must_use]
    pub fn filename(&self) -> Option<&str> {
        match self {
            Self::File(f) => f.filename(),
            _ => None,
        }
    }

    /// Get MIME type if this is a file value
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        match self {
            Self::File(f) => f.mime_type(),
            _ => None,
        }
    }

    /// Check if this is an image file
    #[must_use]
    pub fn is_image(&self) -> bool {
        match self {
            Self::File(f) => f.is_image_file(),
            _ => false,
        }
    }

    /// Check if this is a text file
    #[must_use]
    pub fn is_text(&self) -> bool {
        match self {
            Self::File(f) => f.is_text_file(),
            _ => false,
        }
    }

    // === Conversion methods ===

    /// Try to convert to string representation
    pub fn to_string_representation(&self) -> ValueResult<String> {
        match self {
            Self::String(s) => Ok(s.to_string()),
            Self::File(f) => {
                if f.is_text_file() {
                    f.read_string()
                } else {
                    Ok(format!("File: {}", f.filename().unwrap_or("unnamed")))
                }
            },
        }
    }

    /// Try to get binary data
    pub fn to_binary_data(&self) -> ValueResult<BinaryValue> {
        match self {
            Self::String(s) => Ok(BinaryValue::from(s.as_str())),
            Self::File(f) => f.read_binary(),
        }
    }
}

impl fmt::Display for ModeTypeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s}"),
            Self::File(file) => {
                if let Some(filename) = file.filename() {
                    write!(f, "<file '{}' ({})>", filename, file.file_type())
                } else {
                    write!(f, "<file ({})>", file.file_type())
                }
            },
        }
    }
}

// === From implementations ===

impl From<&str> for ModeTypeValue {
    fn from(s: &str) -> Self {
        Self::string(s)
    }
}

impl From<String> for ModeTypeValue {
    fn from(s: String) -> Self {
        Self::string(s)
    }
}

impl From<StringValue> for ModeTypeValue {
    fn from(s: StringValue) -> Self {
        Self::String(s)
    }
}

impl From<BinaryValue> for ModeTypeValue {
    fn from(b: BinaryValue) -> Self {
        Self::file_from_binary(b, None)
    }
}

impl From<FileValue> for ModeTypeValue {
    fn from(f: FileValue) -> Self {
        Self::File(Box::new(f))
    }
}

impl From<Vec<u8>> for ModeTypeValue {
    fn from(data: Vec<u8>) -> Self {
        Self::file_from_bytes(data, None)
    }
}

// === Default Implementation ===

impl Default for ModeValue {
    fn default() -> Self {
        Self::text("default", "")
    }
}

// === Display Implementation ===

impl fmt::Display for ModeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Pretty format with key info
            write!(f, "{}:({})", self.key, self.value)
        } else {
            // Just the value
            write!(f, "{}", self.value)
        }
    }
}

// === FromStr Implementation ===

impl FromStr for ModeValue {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Default to text mode for string parsing
        Ok(Self::text("parsed", s))
    }
}

// === From implementations for ModeValue ===

impl From<FileValue> for ModeValue {
    fn from(file: FileValue) -> Self {
        Self::file("file_input", file)
    }
}

impl From<BinaryValue> for ModeValue {
    fn from(binary: BinaryValue) -> Self {
        Self::file_binary("binary_input", binary, None)
    }
}

impl TryFrom<&str> for ModeValue {
    type Error = ValueError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self::text("converted", s))
    }
}

impl TryFrom<String> for ModeValue {
    type Error = ValueError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self::text("converted", s))
    }
}

// === JSON Conversion (Ultra-simplified!) ===

#[cfg(feature = "json")]
impl From<ModeValue> for serde_json::Value {
    fn from(mode_value: ModeValue) -> Self {
        serde_json::json!({
            "key": mode_value.key,
            "value": mode_value.value  // Auto-discriminated: String or Object
        })
    }
}

#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for ModeValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::Object(obj) => {
                let key = obj
                    .get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ValueError::custom("Missing key"))?
                    .to_string();

                let value = obj.get("value").ok_or_else(|| ValueError::custom("Missing value"))?;

                // Use #[serde(untagged)] to auto-detect String vs File
                let mode_value = ModeTypeValue::try_from(value.clone())?;

                Ok(ModeValue { key, value: mode_value })
            },
            serde_json::Value::String(s) => {
                // Simple string conversion to text mode
                Ok(ModeValue::text("parsed", s))
            },
            // Legacy support for old format with "mode" field
            other if other.get("mode").is_some() => {
                let mode = other.get("mode").and_then(|v| v.as_str()).unwrap_or("text");
                let key = other.get("key").and_then(|v| v.as_str()).unwrap_or("legacy");
                let value = other.get("value").unwrap_or(&serde_json::Value::Null);

                let mode_type_value = match mode {
                    "text" | "list" => {
                        if let Some(s) = value.as_str() {
                            ModeTypeValue::string(s)
                        } else {
                            return Err(ValueError::custom("Invalid text/list value"));
                        }
                    },
                    "file" => ModeTypeValue::try_from(value.clone())?,
                    _ => return Err(ValueError::custom("Unknown mode type")),
                };

                Ok(ModeValue { key: key.to_string(), value: mode_type_value })
            },
            other => Err(ValueError::custom(format!(
                "Expected object with key and value, got {:?}",
                other
            ))),
        }
    }
}

// Auto-detection for ModeTypeValue using #[serde(untagged)]
#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for ModeTypeValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            // String values become ModeTypeValue::String
            serde_json::Value::String(s) => Ok(ModeTypeValue::string(s)),

            // Object values become ModeTypeValue::File
            serde_json::Value::Object(_) => {
                let file_value = FileValue::try_from(value)?;
                Ok(ModeTypeValue::file(file_value))
            },

            other => Err(ValueError::custom(format!("Expected string or object, got {:?}", other))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_creation() {
        let text_mode = ModeValue::text("user_name", "Alice");
        assert_eq!(text_mode.key(), "user_name");
        assert_eq!(text_mode.as_string(), Some("Alice"));
        assert!(text_mode.is_text_or_list());
        assert!(!text_mode.is_file());

        let file_mode = ModeValue::file_upload(
            "document",
            b"PDF content".to_vec(),
            Some("doc.pdf".to_string()),
        );
        assert_eq!(file_mode.key(), "document");
        assert_eq!(file_mode.filename(), Some("doc.pdf"));
        assert!(!file_mode.is_text_or_list());
        assert!(file_mode.is_file());
    }

    #[test]
    fn test_inferred_modes() {
        let text_param = ModeValue::text("input", "test");
        let file_param = ModeValue::file_upload("upload", b"data".to_vec(), None);

        // Mode is inferred from a value type
        assert_eq!(text_param.inferred_mode(), ModeType::Text);
        assert_eq!(file_param.inferred_mode(), ModeType::File);

        assert!(text_param.is_text_or_list());
        assert!(!text_param.is_file());

        assert!(!file_param.is_text_or_list());
        assert!(file_param.is_file());
    }

    #[test]
    fn test_conversions() {
        let mode = ModeValue::text("greeting", "Hello, World!");

        assert_eq!(mode.to_string_representation().unwrap(), "Hello, World!");

        let binary = mode.to_binary_data().unwrap();
        assert_eq!(binary.as_bytes(), b"Hello, World!");
    }

    #[cfg(all(feature = "serde", feature = "json"))]
    #[test]
    fn test_ultra_simplified_serialization() {
        // Text parameter
        let text_param = ModeValue::text("user_name", "Alice");
        let text_json = serde_json::to_value(&text_param).unwrap();

        // Should be ultra-simple
        assert_eq!(
            text_json,
            serde_json::json!({
                "key": "user_name",
                "value": "Alice"  // Direct string, no "mode" field!
            })
        );

        // File parameter
        let file_param = ModeValue::file_upload(
            "document",
            b"PDF content".to_vec(),
            Some("doc.pdf".to_string()),
        );
        let file_json = serde_json::to_value(&file_param).unwrap();

        // Should be flattened file object
        assert_eq!(file_json["key"], "document");
        assert!(file_json["value"].is_object()); // FileValue object
        assert_eq!(file_json["value"]["type"], "in_memory");
        assert_eq!(file_json["value"]["filename"], "doc.pdf");

        // No "mode" field anywhere!
        assert!(text_json.get("mode").is_none());
        assert!(file_json.get("mode").is_none());
    }

    #[cfg(all(feature = "serde", feature = "json"))]
    #[test]
    fn test_auto_type_detection() {
        // Create from JSON without an explicit mode
        let text_json = serde_json::json!({
            "key": "name",
            "value": "John Doe"  // String -> auto-detected as text
        });

        let text_param: ModeValue = serde_json::from_value(text_json).unwrap();
        assert_eq!(text_param.key(), "name");
        assert_eq!(text_param.as_string(), Some("John Doe"));
        assert!(text_param.is_text_or_list());

        let file_json = serde_json::json!({
            "key": "upload",
            "value": {  // Object -> auto-detected as a file
                "type": "in_memory",
                "data": "SGVsbG8=", // "Hello" in base64
                "filename": "hello.txt"
            }
        });

        let file_param: ModeValue = serde_json::from_value(file_json).unwrap();
        assert_eq!(file_param.key(), "upload");
        assert_eq!(file_param.filename(), Some("hello.txt"));
        assert!(file_param.is_file());
    }

    #[cfg(all(feature = "serde", feature = "json"))]
    #[test]
    fn test_backward_compatibility() {
        // Old format with "mode" field should still work
        let old_format = serde_json::json!({
            "mode": "text",
            "key": "legacy_param",
            "value": "legacy value"
        });

        let param: ModeValue = serde_json::from_value(old_format).unwrap();
        assert_eq!(param.key(), "legacy_param");
        assert_eq!(param.as_string(), Some("legacy value"));

        // When serialized again, should use a new format (no mode field)
        let new_json = serde_json::to_value(&param).unwrap();
        assert!(new_json.get("mode").is_none());
        assert_eq!(new_json["key"], "legacy_param");
        assert_eq!(new_json["value"], "legacy value");
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_file_generated() {
        let params = serde_json::json!({
            "template": "report",
            "data": {"value": 42}
        });

        let file = ModeValue::file_generated(
            "report".to_string(),
            "pdf_generator".to_string(),
            params.clone(),
            Default::default(),
        );

        assert_eq!(file.key(), "report");
        assert!(file.is_file());
    }

    #[test]
    fn test_display_formats() {
        let mode = ModeValue::text("greeting", "Hello");

        // Normal display - just value
        assert_eq!(format!("{}", mode), "Hello");

        // Alternate display - with key
        assert_eq!(format!("{:#}", mode), "greeting:(Hello)");
    }
}
