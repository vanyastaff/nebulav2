use std::fmt;

use chrono::{DateTime, Datelike, Duration as ChronoDuration, NaiveDate, NaiveTime, Timelike, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ValueError, ValueResult};

/// Date and time value supporting different precision levels with rich
/// functionality
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))] // JSON: "2024-01-01T12:00:00Z" or "2024-01-01" or "12:00:00"
pub enum DateTimeValue {
    /// Full date and time with timezone (UTC)
    DateTime(DateTime<Utc>),
    /// Date only (no time component)
    Date(NaiveDate),
    /// Time only (no date component)
    Time(NaiveTime),
}

impl DateTimeValue {
    // === Construction ===

    /// Creates a datetime value for the current moment
    #[inline]
    #[must_use]
    pub fn now() -> Self {
        Self::DateTime(Utc::now())
    }

    /// Creates a datetime value from UTC timestamp (seconds)
    #[must_use]
    pub fn from_timestamp(timestamp: i64) -> Option<Self> {
        DateTime::from_timestamp(timestamp, 0).map(Self::DateTime)
    }

    /// Creates a datetime value from UTC timestamp (milliseconds)
    #[must_use]
    pub fn from_timestamp_millis(millis: i64) -> Option<Self> {
        DateTime::from_timestamp_millis(millis).map(Self::DateTime)
    }

    /// Creates a datetime value from UTC timestamp (microseconds)
    #[must_use]
    pub fn from_timestamp_micros(micros: i64) -> Option<Self> {
        DateTime::from_timestamp_micros(micros).map(Self::DateTime)
    }

    /// Creates a datetime value from UTC timestamp (nanoseconds)
    #[must_use]
    pub fn from_timestamp_nanos(nanos: i64) -> Self {
        Self::DateTime(DateTime::from_timestamp_nanos(nanos))
    }

    /// Creates a date value
    #[must_use]
    pub fn date(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(Self::Date)
    }

    /// Creates a time value
    #[must_use]
    pub fn time(hour: u32, min: u32, sec: u32) -> Option<Self> {
        NaiveTime::from_hms_opt(hour, min, sec).map(Self::Time)
    }

    /// Creates a time value with milliseconds
    #[must_use]
    pub fn time_with_millis(hour: u32, min: u32, sec: u32, millis: u32) -> Option<Self> {
        NaiveTime::from_hms_milli_opt(hour, min, sec, millis).map(Self::Time)
    }

    /// Creates a time value with microseconds
    #[must_use]
    pub fn time_with_micros(hour: u32, min: u32, sec: u32, micros: u32) -> Option<Self> {
        NaiveTime::from_hms_micro_opt(hour, min, sec, micros).map(Self::Time)
    }

    /// Creates a time value with nanoseconds
    #[must_use]
    pub fn time_with_nanos(hour: u32, min: u32, sec: u32, nanos: u32) -> Option<Self> {
        NaiveTime::from_hms_nano_opt(hour, min, sec, nanos).map(Self::Time)
    }

    /// Creates a datetime from date and time components
    #[must_use]
    pub fn from_date_and_time(date: NaiveDate, time: NaiveTime) -> Self {
        Self::DateTime(date.and_time(time).and_utc())
    }

    /// Creates from ISO 8601 string (auto-detects format)
    pub fn from_iso8601(s: &str) -> ValueResult<Self> {
        Self::parse_iso8601(s)
    }

    /// Parse ISO 8601 string with comprehensive format support
    pub fn parse_iso8601(s: &str) -> ValueResult<Self> {
        // Try parsing as full datetime first (RFC 3339)
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(Self::DateTime(dt.with_timezone(&Utc)));
        }

        // Try parsing as date only (YYYY-MM-DD)
        if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            return Ok(Self::Date(date));
        }

        // Try parsing as time only (HH:MM:SS)
        if let Ok(time) = NaiveTime::parse_from_str(s, "%H:%M:%S") {
            return Ok(Self::Time(time));
        }

        // Try parsing as time with milliseconds (HH:MM:SS.sss)
        if let Ok(time) = NaiveTime::parse_from_str(s, "%H:%M:%S%.3f") {
            return Ok(Self::Time(time));
        }

        // Try parsing as time with microseconds (HH:MM:SS.ssssss)
        if let Ok(time) = NaiveTime::parse_from_str(s, "%H:%M:%S%.6f") {
            return Ok(Self::Time(time));
        }

        // Try other common datetime formats
        let formats = [
            "%Y-%m-%d %H:%M:%S %z",
            "%Y-%m-%dT%H:%M:%S",
            "%Y-%m-%d %H:%M:%S",
            "%d/%m/%Y %H:%M:%S",
            "%m/%d/%Y %H:%M:%S",
        ];

        for format in &formats {
            if let Ok(dt) = DateTime::parse_from_str(s, format) {
                return Ok(Self::DateTime(dt.with_timezone(&Utc)));
            }
        }

        // Try date-only formats
        let date_formats = ["%d/%m/%Y", "%m/%d/%Y", "%Y/%m/%d", "%d-%m-%Y", "%m-%d-%Y"];

        for format in &date_formats {
            if let Ok(date) = NaiveDate::parse_from_str(s, format) {
                return Ok(Self::Date(date));
            }
        }

        Err(ValueError::custom(format!(
            "Unable to parse datetime from string: '{}'. Supported formats include ISO 8601, RFC 3339, and common date/time patterns.",
            s
        )))
    }

    // === Type Checks ===

    /// Returns true if this is a datetime value
    #[inline]
    #[must_use]
    pub const fn is_datetime(&self) -> bool {
        matches!(self, Self::DateTime(_))
    }

    /// Returns true if this is a date-only value
    #[inline]
    #[must_use]
    pub const fn is_date(&self) -> bool {
        matches!(self, Self::Date(_))
    }

    /// Returns true if this is a time-only value
    #[inline]
    #[must_use]
    pub const fn is_time(&self) -> bool {
        matches!(self, Self::Time(_))
    }

    // === Component Access ===

    /// Gets the date component (if available)
    #[must_use]
    pub fn date_component(&self) -> Option<NaiveDate> {
        match self {
            Self::DateTime(dt) => Some(dt.date_naive()),
            Self::Date(date) => Some(*date),
            Self::Time(_) => None,
        }
    }

    /// Gets the time component (if available)
    #[must_use]
    pub fn time_component(&self) -> Option<NaiveTime> {
        match self {
            Self::DateTime(dt) => Some(dt.time()),
            Self::Date(_) => None,
            Self::Time(time) => Some(*time),
        }
    }

    /// Gets the year (if available)
    #[must_use]
    pub fn year(&self) -> Option<i32> {
        match self {
            Self::DateTime(dt) => Some(dt.year()),
            Self::Date(date) => Some(date.year()),
            Self::Time(_) => None,
        }
    }

    /// Gets the month (1-12) (if available)
    #[must_use]
    pub fn month(&self) -> Option<u32> {
        match self {
            Self::DateTime(dt) => Some(dt.month()),
            Self::Date(date) => Some(date.month()),
            Self::Time(_) => None,
        }
    }

    /// Gets the day of month (1-31) (if available)
    #[must_use]
    pub fn day(&self) -> Option<u32> {
        match self {
            Self::DateTime(dt) => Some(dt.day()),
            Self::Date(date) => Some(date.day()),
            Self::Time(_) => None,
        }
    }

    /// Gets the hour (0-23) (if available)
    #[must_use]
    pub fn hour(&self) -> Option<u32> {
        match self {
            Self::DateTime(dt) => Some(dt.hour()),
            Self::Date(_) => None,
            Self::Time(time) => Some(time.hour()),
        }
    }

    /// Gets the minute (0-59) (if available)
    #[must_use]
    pub fn minute(&self) -> Option<u32> {
        match self {
            Self::DateTime(dt) => Some(dt.minute()),
            Self::Date(_) => None,
            Self::Time(time) => Some(time.minute()),
        }
    }

    /// Gets the second (0-59) (if available)
    #[must_use]
    pub fn second(&self) -> Option<u32> {
        match self {
            Self::DateTime(dt) => Some(dt.second()),
            Self::Date(_) => None,
            Self::Time(time) => Some(time.second()),
        }
    }

    // === Timestamp Methods ===

    /// Returns the timestamp in seconds (datetime only)
    #[must_use]
    pub fn timestamp(&self) -> Option<i64> {
        match self {
            Self::DateTime(dt) => Some(dt.timestamp()),
            _ => None,
        }
    }

    /// Returns the timestamp in milliseconds (datetime only)
    #[must_use]
    pub fn timestamp_millis(&self) -> Option<i64> {
        match self {
            Self::DateTime(dt) => Some(dt.timestamp_millis()),
            _ => None,
        }
    }

    /// Returns the timestamp in microseconds (datetime only)
    #[must_use]
    pub fn timestamp_micros(&self) -> Option<i64> {
        match self {
            Self::DateTime(dt) => Some(dt.timestamp_micros()),
            _ => None,
        }
    }

    /// Returns the timestamp in nanoseconds (datetime only)
    #[must_use]
    pub fn timestamp_nanos(&self) -> Option<i64> {
        match self {
            Self::DateTime(dt) => dt.timestamp_nanos_opt(),
            _ => None,
        }
    }

    // === Conversion Methods ===

    /// Converts to full datetime (requires a date for time-only values)
    #[must_use]
    pub fn to_datetime(&self, default_date: Option<NaiveDate>) -> Option<DateTime<Utc>> {
        match self {
            Self::DateTime(dt) => Some(*dt),
            Self::Date(date) => Some(date.and_hms_opt(0, 0, 0)?.and_utc()),
            Self::Time(time) => {
                let date = default_date?;
                Some(date.and_time(*time).and_utc())
            },
        }
    }

    /// Converts to full datetime using current date for time-only values
    #[must_use]
    pub fn to_datetime_with_current_date(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::DateTime(dt) => Some(*dt),
            Self::Date(date) => Some(date.and_hms_opt(0, 0, 0)?.and_utc()),
            Self::Time(time) => {
                let today = Utc::now().date_naive();
                Some(today.and_time(*time).and_utc())
            },
        }
    }

    /// Converts to NaiveDate (time component is ignored for datetime)
    #[must_use]
    pub fn to_date(&self) -> Option<NaiveDate> {
        self.date_component()
    }

    /// Converts to NaiveTime (date component is ignored for datetime)
    #[must_use]
    pub fn to_time(&self) -> Option<NaiveTime> {
        self.time_component()
    }

    // === Formatting Methods ===

    /// Converts to ISO 8601 string
    #[must_use]
    pub fn to_iso8601(&self) -> String {
        match self {
            Self::DateTime(dt) => dt.to_rfc3339(),
            Self::Date(date) => date.format("%Y-%m-%d").to_string(),
            Self::Time(time) => time.format("%H:%M:%S").to_string(),
        }
    }

    /// Format as human-readable string
    #[must_use]
    pub fn format_human(&self) -> String {
        match self {
            Self::DateTime(dt) => dt.format("%B %d, %Y at %I:%M:%S %p UTC").to_string(),
            Self::Date(date) => date.format("%B %d, %Y").to_string(),
            Self::Time(time) => time.format("%I:%M:%S %p").to_string(),
        }
    }

    /// Format with custom pattern
    pub fn format_custom(&self, pattern: &str) -> ValueResult<String> {
        match self {
            Self::DateTime(dt) => Ok(dt.format(pattern).to_string()),
            Self::Date(date) => Ok(date.format(pattern).to_string()),
            Self::Time(time) => Ok(time.format(pattern).to_string()),
        }
    }

    /// Format as compact string (e.g., "2024-01-15", "14:30:45")
    #[must_use]
    pub fn format_compact(&self) -> String {
        match self {
            Self::DateTime(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            Self::Date(date) => date.format("%Y-%m-%d").to_string(),
            Self::Time(time) => time.format("%H:%M:%S").to_string(),
        }
    }

    // === Date/Time Arithmetic ===

    /// Add duration to datetime (only works for datetime values)
    pub fn add_duration(&self, duration: ChronoDuration) -> ValueResult<Self> {
        match self {
            Self::DateTime(dt) => dt
                .checked_add_signed(duration)
                .map(Self::DateTime)
                .ok_or_else(|| ValueError::custom("Duration addition would overflow")),
            _ => Err(ValueError::custom("Can only add duration to datetime values")),
        }
    }

    /// Subtract duration from datetime (only works for datetime values)
    pub fn sub_duration(&self, duration: ChronoDuration) -> ValueResult<Self> {
        match self {
            Self::DateTime(dt) => dt
                .checked_sub_signed(duration)
                .map(Self::DateTime)
                .ok_or_else(|| ValueError::custom("Duration subtraction would underflow")),
            _ => Err(ValueError::custom("Can only subtract duration from datetime values")),
        }
    }

    /// Calculate duration between two datetime values
    pub fn duration_since(&self, other: &Self) -> ValueResult<ChronoDuration> {
        match (self, other) {
            (Self::DateTime(dt1), Self::DateTime(dt2)) => Ok(*dt1 - *dt2),
            _ => Err(ValueError::custom("Can only calculate duration between datetime values")),
        }
    }

    // === Validation Methods ===

    /// Check if the datetime is in the past
    #[must_use]
    pub fn is_past(&self) -> bool {
        match self {
            Self::DateTime(dt) => *dt < Utc::now(),
            _ => false, // Date/Time only values can't be compared to "now"
        }
    }

    /// Check if the datetime is in the future
    #[must_use]
    pub fn is_future(&self) -> bool {
        match self {
            Self::DateTime(dt) => *dt > Utc::now(),
            _ => false,
        }
    }

    /// Check if the datetime is today (only for date or datetime)
    #[must_use]
    pub fn is_today(&self) -> bool {
        let today = Utc::now().date_naive();
        match self {
            Self::DateTime(dt) => dt.date_naive() == today,
            Self::Date(date) => *date == today,
            Self::Time(_) => false,
        }
    }

    /// Check if two values represent the same date (ignoring time)
    #[must_use]
    pub fn same_date(&self, other: &Self) -> bool {
        match (self.date_component(), other.date_component()) {
            (Some(d1), Some(d2)) => d1 == d2,
            _ => false,
        }
    }

    /// Check if two values represent the same time (ignoring date)
    #[must_use]
    pub fn same_time(&self, other: &Self) -> bool {
        match (self.time_component(), other.time_component()) {
            (Some(t1), Some(t2)) => t1 == t2,
            _ => false,
        }
    }

    /// Check if the datetime is within a reasonable range for workflow
    /// scheduling
    #[must_use]
    pub fn is_reasonable_schedule_time(&self) -> bool {
        match self {
            Self::DateTime(dt) => {
                let now = Utc::now();
                let one_year_from_now = now + ChronoDuration::days(365);
                *dt >= now && *dt <= one_year_from_now
            },
            _ => true, // Date/Time only values are generally reasonable
        }
    }
}

// === Default Implementation ===

impl Default for DateTimeValue {
    #[inline]
    fn default() -> Self {
        Self::now()
    }
}

// === Display Implementation ===

impl fmt::Display for DateTimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Pretty format with #
            write!(f, "{}", self.format_human())
        } else {
            // Compact format
            write!(f, "{}", self.format_compact())
        }
    }
}

// === From Implementations ===

impl From<DateTime<Utc>> for DateTimeValue {
    #[inline]
    fn from(value: DateTime<Utc>) -> Self {
        Self::DateTime(value)
    }
}

impl From<NaiveDate> for DateTimeValue {
    #[inline]
    fn from(value: NaiveDate) -> Self {
        Self::Date(value)
    }
}

impl From<NaiveTime> for DateTimeValue {
    #[inline]
    fn from(value: NaiveTime) -> Self {
        Self::Time(value)
    }
}

// === JSON Conversion ===
#[cfg(feature = "json")]
impl From<DateTimeValue> for serde_json::Value {
    fn from(datetime: DateTimeValue) -> Self {
        serde_json::Value::String(datetime.to_iso8601())
    }
}
#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for DateTimeValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::String(s) => DateTimeValue::parse_iso8601(&s),
            serde_json::Value::Number(n) => {
                if let Some(timestamp) = n.as_i64() {
                    DateTimeValue::from_timestamp(timestamp).ok_or_else(|| {
                        ValueError::type_conversion_with_value(
                            n.to_string(),
                            "DateTimeValue",
                            "invalid timestamp".to_string(),
                        )
                    })
                } else {
                    Err(ValueError::type_conversion_with_value(
                        n.to_string(),
                        "DateTimeValue",
                        "timestamp must be a valid integer".to_string(),
                    ))
                }
            },
            other => Err(ValueError::type_conversion_with_value(
                format!("{:?}", other),
                "DateTimeValue",
                "expected string or number".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let now = DateTimeValue::now();
        assert!(now.is_datetime());

        let date = DateTimeValue::date(2024, 1, 15).unwrap();
        assert!(date.is_date());
        assert_eq!(date.year(), Some(2024));

        let time = DateTimeValue::time(14, 30, 45).unwrap();
        assert!(time.is_time());
        assert_eq!(time.hour(), Some(14));
    }

    #[test]
    fn test_parsing() {
        let dt = DateTimeValue::parse_iso8601("2024-01-15T14:30:45Z").unwrap();
        assert!(dt.is_datetime());

        let date = DateTimeValue::parse_iso8601("2024-01-15").unwrap();
        assert!(date.is_date());

        let time = DateTimeValue::parse_iso8601("14:30:45").unwrap();
        assert!(time.is_time());
    }

    #[test]
    fn test_formatting() {
        let dt = DateTimeValue::from_timestamp(1705329045).unwrap(); // 2024-01-15 14:30:45 UTC

        assert!(dt.format_compact().contains("2024-01-15"));
        assert!(dt.format_human().contains("January"));
    }

    #[test]
    fn test_arithmetic() {
        let dt = DateTimeValue::from_timestamp(1705329045).unwrap();
        let duration = ChronoDuration::hours(1);

        let later = dt.add_duration(duration).unwrap();
        assert!(later.timestamp().unwrap() > dt.timestamp().unwrap());
    }

    #[test]
    fn test_validation() {
        let past = DateTimeValue::from_timestamp(946684800).unwrap(); // Year 2000
        assert!(past.is_past());
        assert!(!past.is_future());
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_json_conversion() {
        let dt = DateTimeValue::date(2024, 1, 15).unwrap();
        let json: serde_json::Value = dt.into();
        assert_eq!(json, serde_json::Value::String("2024-01-15".to_string()));

        let parsed: DateTimeValue =
            serde_json::Value::String("2024-01-15".to_string()).try_into().unwrap();
        assert_eq!(parsed, dt);
    }
}
