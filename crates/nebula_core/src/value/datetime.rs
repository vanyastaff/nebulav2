use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Date and time value supporting different precision levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Display)]
#[serde(untagged)] // JSON: "2024-01-01T12:00:00Z" or "2024-01-01" or "12:00:00"
pub enum DateTimeValue {
    /// Full date and time with timezone (UTC)
    DateTime(DateTime<Utc>),
    /// Date only (no time component)
    Date(NaiveDate),
    /// Time only (no date component)
    Time(NaiveTime),
}

impl DateTimeValue {
    /// Creates a datetime value for the current moment
    pub fn now() -> Self {
        DateTimeValue::DateTime(Utc::now())
    }

    /// Creates a datetime value from UTC timestamp
    pub fn from_timestamp(timestamp: i64) -> Option<Self> {
        DateTime::from_timestamp(timestamp, 0).map(DateTimeValue::DateTime)
    }

    /// Creates a date value
    pub fn date(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(DateTimeValue::Date)
    }

    /// Creates a time value
    pub fn time(hour: u32, min: u32, sec: u32) -> Option<Self> {
        NaiveTime::from_hms_opt(hour, min, sec).map(DateTimeValue::Time)
    }

    /// Creates from ISO 8601 string (auto-detects format)
    pub fn from_iso8601(s: &str) -> Result<Self, chrono::ParseError> {
        // Try parsing as full datetime first
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(DateTimeValue::DateTime(dt.with_timezone(&Utc)));
        }

        // Try parsing as date only
        if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            return Ok(DateTimeValue::Date(date));
        }

        // Try parsing as time only
        if let Ok(time) = NaiveTime::parse_from_str(s, "%H:%M:%S") {
            return Ok(DateTimeValue::Time(time));
        }

        // If all fail, try other common formats
        DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S %z")
            .map(|dt| DateTimeValue::DateTime(dt.with_timezone(&Utc)))
    }

    /// Converts to ISO 8601 string
    pub fn to_iso8601(&self) -> String {
        match self {
            DateTimeValue::DateTime(dt) => dt.to_rfc3339(),
            DateTimeValue::Date(date) => date.format("%Y-%m-%d").to_string(),
            DateTimeValue::Time(time) => time.format("%H:%M:%S").to_string(),
        }
    }

    /// Returns true if this is a datetime value
    pub fn is_datetime(&self) -> bool {
        matches!(self, DateTimeValue::DateTime(_))
    }

    /// Returns true if this is a date-only value
    pub fn is_date(&self) -> bool {
        matches!(self, DateTimeValue::Date(_))
    }

    /// Returns true if this is a time-only value
    pub fn is_time(&self) -> bool {
        matches!(self, DateTimeValue::Time(_))
    }

    /// Gets the date component (if available)
    pub fn date_component(&self) -> Option<NaiveDate> {
        match self {
            DateTimeValue::DateTime(dt) => Some(dt.date_naive()),
            DateTimeValue::Date(date) => Some(*date),
            DateTimeValue::Time(_) => None,
        }
    }

    /// Gets the time component (if available)
    pub fn time_component(&self) -> Option<NaiveTime> {
        match self {
            DateTimeValue::DateTime(dt) => Some(dt.time()),
            DateTimeValue::Date(_) => None,
            DateTimeValue::Time(time) => Some(*time),
        }
    }

    /// Returns the timestamp in seconds (datetime only)
    pub fn timestamp(&self) -> Option<i64> {
        match self {
            DateTimeValue::DateTime(dt) => Some(dt.timestamp()),
            _ => None,
        }
    }

    /// Returns the timestamp in milliseconds (datetime only)
    pub fn timestamp_millis(&self) -> Option<i64> {
        match self {
            DateTimeValue::DateTime(dt) => Some(dt.timestamp_millis()),
            _ => None,
        }
    }

    /// Converts to full datetime (requires a date for time-only values)
    pub fn to_datetime(&self, default_date: Option<NaiveDate>) -> Option<DateTime<Utc>> {
        match self {
            DateTimeValue::DateTime(dt) => Some(*dt),
            DateTimeValue::Date(date) => Some(date.and_hms_opt(0, 0, 0)?.and_utc()),
            DateTimeValue::Time(time) => {
                let date = default_date?;
                Some(date.and_time(*time).and_utc())
            }
        }
    }
}

// Convenient From implementations
impl From<DateTime<Utc>> for DateTimeValue {
    fn from(value: DateTime<Utc>) -> Self {
        DateTimeValue::DateTime(value)
    }
}

impl From<NaiveDate> for DateTimeValue {
    fn from(value: NaiveDate) -> Self {
        DateTimeValue::Date(value)
    }
}

impl From<NaiveTime> for DateTimeValue {
    fn from(value: NaiveTime) -> Self {
        DateTimeValue::Time(value)
    }
}

impl Into<serde_json::Value> for DateTimeValue {
    fn into(self) -> serde_json::Value {
        serde_json::Value::String(self.to_iso8601())
    }
}
