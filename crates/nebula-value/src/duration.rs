#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ValueError, ValueResult};
use std::fmt;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};
use std::time::Duration;

/// Duration value for time intervals with rich functionality
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DurationValue(Duration);

impl DurationValue {
    // === Construction ===

    /// Creates a new duration value
    #[inline]
    #[must_use]
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Creates a zero duration
    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self(Duration::ZERO)
    }

    /// Creates a maximum duration
    #[inline]
    #[must_use]
    pub const fn maximum() -> Self {
        Self(Duration::MAX)
    }

    /// Creates a duration from seconds
    #[inline]
    #[must_use]
    pub const fn from_secs(secs: u64) -> Self {
        Self(Duration::from_secs(secs))
    }

    /// Creates a duration from milliseconds
    #[inline]
    #[must_use]
    pub const fn from_millis(millis: u64) -> Self {
        Self(Duration::from_millis(millis))
    }

    /// Creates a duration from microseconds
    #[inline]
    #[must_use]
    pub const fn from_micros(micros: u64) -> Self {
        Self(Duration::from_micros(micros))
    }

    /// Creates a duration from nanoseconds
    #[inline]
    #[must_use]
    pub const fn from_nanos(nanos: u64) -> Self {
        Self(Duration::from_nanos(nanos))
    }

    /// Creates a duration from floating-point seconds
    pub fn from_secs_f64(secs: f64) -> ValueResult<Self> {
        if !secs.is_finite() || secs < 0.0 {
            return Err(ValueError::custom(format!(
                "Invalid duration: {} seconds must be finite and non-negative",
                secs
            )));
        }
        Ok(Self(Duration::from_secs_f64(secs)))
    }

    /// Creates a duration from floating-point seconds (unchecked)
    #[inline]
    #[must_use]
    pub fn from_secs_f64_unchecked(secs: f64) -> Self {
        Self(Duration::from_secs_f64(secs))
    }

    /// Creates a duration from floating-point seconds (32-bit)
    pub fn from_secs_f32(secs: f32) -> ValueResult<Self> {
        if !secs.is_finite() || secs < 0.0 {
            return Err(ValueError::custom(format!(
                "Invalid duration: {} seconds must be finite and non-negative",
                secs
            )));
        }
        Ok(Self(Duration::from_secs_f32(secs)))
    }

    // === Common Time Units ===

    /// Creates a duration representing minutes
    #[inline]
    #[must_use]
    pub const fn from_minutes(minutes: u64) -> Self {
        Self(Duration::from_secs(minutes * 60))
    }

    /// Creates a duration representing hours
    #[inline]
    #[must_use]
    pub const fn from_hours(hours: u64) -> Self {
        Self(Duration::from_secs(hours * 3600))
    }

    /// Creates a duration representing days
    #[inline]
    #[must_use]
    pub const fn from_days(days: u64) -> Self {
        Self(Duration::from_secs(days * 86400))
    }

    // === Access Methods ===

    /// Returns the duration in seconds
    #[inline]
    #[must_use]
    pub const fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }

    /// Returns the duration in milliseconds
    #[inline]
    #[must_use]
    pub const fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    /// Returns the duration in microseconds
    #[inline]
    #[must_use]
    pub const fn as_micros(&self) -> u128 {
        self.0.as_micros()
    }

    /// Returns the duration in nanoseconds
    #[inline]
    #[must_use]
    pub const fn as_nanos(&self) -> u128 {
        self.0.as_nanos()
    }

    /// Returns the duration as floating-point seconds
    #[inline]
    #[must_use]
    pub fn as_secs_f64(&self) -> f64 {
        self.0.as_secs_f64()
    }

    /// Returns the duration as floating-point seconds (32-bit)
    #[inline]
    #[must_use]
    pub fn as_secs_f32(&self) -> f32 {
        self.0.as_secs_f32()
    }

    /// Returns the duration in minutes (truncated)
    #[inline]
    #[must_use]
    pub const fn as_minutes(&self) -> u64 {
        self.as_secs() / 60
    }

    /// Returns the duration in hours (truncated)
    #[inline]
    #[must_use]
    pub const fn as_hours(&self) -> u64 {
        self.as_secs() / 3600
    }

    /// Returns the duration in days (truncated)
    #[inline]
    #[must_use]
    pub const fn as_days(&self) -> u64 {
        self.as_secs() / 86400
    }

    /// Returns the subsecond nanoseconds
    #[inline]
    #[must_use]
    pub const fn subsec_nanos(&self) -> u32 {
        self.0.subsec_nanos()
    }

    /// Returns the subsecond microseconds
    #[inline]
    #[must_use]
    pub const fn subsec_micros(&self) -> u32 {
        self.0.subsec_micros()
    }

    /// Returns the subsecond milliseconds
    #[inline]
    #[must_use]
    pub const fn subsec_millis(&self) -> u32 {
        self.0.subsec_millis()
    }

    // === State Checks ===

    /// Returns true if the duration is zero
    #[inline]
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Returns true if the duration is maximum
    #[inline]
    #[must_use]
    pub fn is_maximum(&self) -> bool {
        *self == Self::maximum()
    }

    // === Validation Methods ===

    /// Checks if duration is within reasonable timeout range
    #[must_use]
    pub fn is_reasonable_timeout(&self) -> bool {
        // Between 1ms and 1 hour
        self.as_millis() >= 1 && self.as_secs() <= 3600
    }

    /// Checks if duration exceeds a maximum limit
    #[inline]
    #[must_use]
    pub fn exceeds(&self, max: &Self) -> bool {
        self > max
    }

    /// Checks if duration is below a minimum limit
    #[inline]
    #[must_use]
    pub fn below(&self, min: &Self) -> bool {
        self < min
    }

    /// Checks if duration is within a range (inclusive)
    #[inline]
    #[must_use]
    pub fn within_range(&self, min: &Self, max: &Self) -> bool {
        self >= min && self <= max
    }

    // === Mathematical Operations ===

    /// Saturating addition - won't overflow
    #[inline]
    #[must_use]
    pub fn saturating_add(&self, other: &Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    /// Saturating subtraction - won't underflow
    #[inline]
    #[must_use]
    pub fn saturating_sub(&self, other: &Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

    /// Checked addition
    #[inline]
    #[must_use]
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    /// Checked subtraction
    #[inline]
    #[must_use]
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }

    /// Checked multiplication by integer
    #[inline]
    #[must_use]
    pub fn checked_mul(&self, rhs: u32) -> Option<Self> {
        self.0.checked_mul(rhs).map(Self)
    }

    /// Checked division by integer
    #[inline]
    #[must_use]
    pub fn checked_div(&self, rhs: u32) -> Option<Self> {
        self.0.checked_div(rhs).map(Self)
    }

    /// Multiply by floating-point factor
    pub fn multiply_f64(&self, factor: f64) -> ValueResult<Self> {
        if !factor.is_finite() || factor < 0.0 {
            return Err(ValueError::custom(format!(
                "Invalid factor: {} must be finite and non-negative",
                factor
            )));
        }

        let new_secs = self.as_secs_f64() * factor;
        Self::from_secs_f64(new_secs)
    }

    /// Calculate percentage of duration
    pub fn percentage(&self, percent: f64) -> ValueResult<Self> {
        if !percent.is_finite() {
            return Err(ValueError::custom("Percentage must be finite"));
        }
        self.multiply_f64(percent / 100.0)
    }

    /// Divide into equal parts
    pub fn divide_into(&self, parts: u32) -> ValueResult<Vec<Self>> {
        if parts == 0 {
            return Err(ValueError::custom("Cannot divide into zero parts"));
        }

        let each = self
            .checked_div(parts)
            .ok_or_else(|| ValueError::custom("Duration too small to divide"))?;

        Ok(vec![each; parts as usize])
    }

    // === Utility Methods ===

    /// Returns the minimum of two durations
    #[inline]
    #[must_use]
    pub fn min(&self, other: &Self) -> Self {
        Self(self.0.min(other.0))
    }

    /// Returns the maximum of two durations
    #[inline]
    #[must_use]
    pub fn max(&self, other: &Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// Clamps duration to a range
    #[inline]
    #[must_use]
    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self(self.0.clamp(min.0, max.0))
    }

    /// Returns the absolute difference between two durations
    #[inline]
    #[must_use]
    pub fn abs_diff(&self, other: &Self) -> Self {
        Self(self.0.abs_diff(other.0))
    }

    // === Formatting Methods ===

    /// Format as compact string (e.g., "1.5s", "30ms")
    #[must_use]
    pub fn format_compact(&self) -> String {
        if self.is_zero() {
            return "0s".to_string();
        }

        let total_nanos = self.as_nanos();

        // Choose the most appropriate unit
        if total_nanos >= 1_000_000_000 {
            // >= 1 second
            let secs = self.as_secs_f64();
            if secs >= 60.0 { format!("{:.1}m", secs / 60.0) } else { format!("{:.1}s", secs) }
        } else if total_nanos >= 1_000_000 {
            // >= 1 millisecond
            format!("{}ms", self.as_millis())
        } else if total_nanos >= 1_000 {
            // >= 1 microsecond
            format!("{}μs", self.as_micros())
        } else {
            format!("{}ns", total_nanos)
        }
    }

    /// Format as verbose string (e.g., "1 hour 30 minutes", "45 seconds")
    #[must_use]
    #[allow(clippy::cognitive_complexity)]
    pub fn format_verbose(&self) -> String {
        if self.is_zero() {
            return "0 seconds".to_string();
        }

        let mut parts = Vec::new();
        let mut remaining = *self; // Copy the value instead of moving

        // Days
        let days = remaining.as_days();
        if days > 0 {
            parts.push(format!("{} {}", days, if days == 1 { "day" } else { "days" }));
            remaining = remaining.saturating_sub(&Self::from_days(days));
        }

        // Hours
        let hours = remaining.as_hours();
        if hours > 0 {
            parts.push(format!("{} {}", hours, if hours == 1 { "hour" } else { "hours" }));
            remaining = remaining.saturating_sub(&Self::from_hours(hours));
        }

        // Minutes
        let minutes = remaining.as_minutes();
        if minutes > 0 {
            parts.push(format!("{} {}", minutes, if minutes == 1 { "minute" } else { "minutes" }));
            remaining = remaining.saturating_sub(&Self::from_minutes(minutes));
        }

        // Seconds
        let secs = remaining.as_secs();
        if secs > 0 || parts.is_empty() {
            parts.push(format!("{} {}", secs, if secs == 1 { "second" } else { "seconds" }));
        }

        // Join with appropriate separators
        match parts.len() {
            1 => parts[0].clone(),
            2 => format!("{} and {}", parts[0], parts[1]),
            _ => {
                let last = parts.pop().unwrap();
                format!("{}, and {}", parts.join(", "), last)
            },
        }
    }

    /// Convert to std::time::Duration
    #[inline]
    #[must_use]
    pub const fn as_std_duration(&self) -> Duration {
        self.0
    }

    /// Convert to std::time::Duration (consuming)
    #[inline]
    #[must_use]
    pub fn into_std_duration(self) -> Duration {
        self.0
    }
}

// === Default Implementation ===

impl Default for DurationValue {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl TryFrom<f64> for DurationValue {
    type Error = ValueError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0.0 {
            Err(ValueError::type_conversion_with_value(
                value.to_string(),
                "DurationValue",
                "negative duration is not allowed",
            ))
        } else {
            Ok(DurationValue::from_secs_f64(value)?)
        }
    }
}

// === Display Implementation ===

impl fmt::Display for DurationValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Pretty format with #
            write!(f, "{}", self.format_verbose())
        } else {
            // Compact format
            write!(f, "{}", self.format_compact())
        }
    }
}

// === JSON Conversion ===
#[cfg(feature = "json")]
impl From<DurationValue> for serde_json::Value {
    fn from(duration: DurationValue) -> Self {
        serde_json::Value::Number(serde_json::Number::from(duration.as_millis() as u64))
    }
}

#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for DurationValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::Number(n) => {
                if let Some(ms) = n.as_u64() {
                    Ok(DurationValue::from_millis(ms))
                } else if let Some(f) = n.as_f64() {
                    DurationValue::from_secs_f64(f / 1000.0)
                } else {
                    Err(ValueError::type_conversion_with_value(
                        n.to_string(),
                        "DurationValue",
                        "number must be non-negative".to_string(),
                    ))
                }
            },
            serde_json::Value::String(s) => {
                // Simple string parsing: "1000ms", "5s", "2.5s"
                if let Some(stripped) = s.strip_suffix("ms") {
                    stripped.parse::<u64>().map(DurationValue::from_millis).map_err(|_| {
                        ValueError::type_conversion_with_value(
                            s.clone(),
                            "DurationValue",
                            "invalid milliseconds format".to_string(),
                        )
                    })
                } else if let Some(stripped) = s.strip_suffix("s") {
                    stripped
                        .parse::<f64>()
                        .map_err(|_| {
                            ValueError::type_conversion_with_value(
                                s.clone(),
                                "DurationValue",
                                "invalid seconds format".to_string(),
                            )
                        })
                        .and_then(DurationValue::from_secs_f64)
                } else {
                    Err(ValueError::type_conversion_with_value(
                        s,
                        "DurationValue",
                        "unsupported format - use '5s' or '1000ms'".to_string(),
                    ))
                }
            },
            other => Err(ValueError::type_conversion_with_value(
                format!("{:?}", other),
                "DurationValue",
                "expected number or string".to_string(),
            )),
        }
    }
}

// === Iterator Support for divide_into ===

impl IntoIterator for DurationValue {
    type Item = DurationValue;
    type IntoIter = std::iter::Once<DurationValue>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

// === Manual Trait Implementations (no derive_more) ===

impl From<Duration> for DurationValue {
    #[inline]
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl From<DurationValue> for Duration {
    #[inline]
    fn from(duration: DurationValue) -> Self {
        duration.0
    }
}

impl Deref for DurationValue {
    type Target = Duration;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DurationValue {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for DurationValue {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<&DurationValue> for DurationValue {
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Duration> for DurationValue {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub for DurationValue {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<&DurationValue> for DurationValue {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<Duration> for DurationValue {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Mul<u32> for DurationValue {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<DurationValue> for u32 {
    type Output = DurationValue;

    #[inline]
    fn mul(self, rhs: DurationValue) -> Self::Output {
        DurationValue(rhs.0 * self)
    }
}

impl Div<u32> for DurationValue {
    type Output = Self;

    #[inline]
    fn div(self, rhs: u32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

// Additional assignment operators
impl std::ops::AddAssign for DurationValue {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::AddAssign<Duration> for DurationValue {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs;
    }
}

impl std::ops::SubAssign for DurationValue {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::SubAssign<Duration> for DurationValue {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= rhs;
    }
}

impl std::ops::MulAssign<u32> for DurationValue {
    #[inline]
    fn mul_assign(&mut self, rhs: u32) {
        self.0 *= rhs;
    }
}

impl std::ops::DivAssign<u32> for DurationValue {
    #[inline]
    fn div_assign(&mut self, rhs: u32) {
        self.0 /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        assert_eq!(DurationValue::zero().as_nanos(), 0);
        assert_eq!(DurationValue::from_secs(5).as_secs(), 5);
        assert_eq!(DurationValue::from_millis(1500).as_millis(), 1500);
        assert_eq!(DurationValue::from_minutes(2).as_secs(), 120);
        assert_eq!(DurationValue::from_hours(1).as_secs(), 3600);
        assert_eq!(DurationValue::from_days(1).as_secs(), 86400);
    }

    #[test]
    fn test_formatting() {
        assert_eq!(DurationValue::zero().format_compact(), "0s");
        assert_eq!(DurationValue::from_millis(500).format_compact(), "500ms");
        assert_eq!(DurationValue::from_secs(1).format_compact(), "1.0s");
        assert_eq!(DurationValue::from_secs(90).format_compact(), "1.5m");

        assert_eq!(DurationValue::from_secs(1).format_verbose(), "1 second");
        assert_eq!(DurationValue::from_secs(61).format_verbose(), "1 minute and 1 second");
    }

    #[test]
    fn test_validation() {
        let timeout = DurationValue::from_millis(5000);
        assert!(timeout.is_reasonable_timeout());

        let too_long = DurationValue::from_hours(2);
        assert!(!too_long.is_reasonable_timeout());
    }

    #[test]
    fn test_mathematical_operations() {
        let dur = DurationValue::from_secs(10);

        let doubled = dur.multiply_f64(2.0).unwrap();
        assert_eq!(doubled.as_secs(), 20);

        let quarter = dur.percentage(25.0).unwrap();
        assert_eq!(quarter.as_millis(), 2500);

        let parts = dur.divide_into(4).unwrap();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0].as_millis(), 2500);
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_json_conversion() {
        let dur = DurationValue::from_millis(1500);
        let json: serde_json::Value = dur.into();
        assert_eq!(json, serde_json::Value::Number(1500.into()));

        let parsed: DurationValue =
            serde_json::Value::String("2.5s".to_string()).try_into().unwrap();
        assert_eq!(parsed.as_millis(), 2500);
    }
}
