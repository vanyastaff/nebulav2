#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ValueError, ValueResult};
use chrono::{DateTime, Datelike, Duration as ChronoDuration, Timelike, Utc};
use std::fmt;
use std::str::FromStr;

/// Cron expression value for scheduling with rich functionality
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CronValue {
    expression: String,
    parsed: CronExpression,
}

impl CronValue {
    // === Construction ===

    /// Creates a new cron value from an expression string
    pub fn new(expression: impl AsRef<str>) -> ValueResult<Self> {
        let expr_str = expression.as_ref().trim();
        let parsed = CronExpression::parse(expr_str)?;

        Ok(Self { expression: expr_str.to_string(), parsed })
    }

    /// Creates a cron expression that runs every minute
    #[must_use]
    pub fn every_minute() -> Self {
        Self::new("* * * * *").unwrap()
    }

    /// Creates a cron expression that runs every hour at minute 0
    #[must_use]
    pub fn every_hour() -> Self {
        Self::new("0 * * * *").unwrap()
    }

    /// Creates a cron expression that runs daily at midnight
    #[must_use]
    pub fn daily() -> Self {
        Self::new("0 0 * * *").unwrap()
    }

    /// Creates a cron expression that runs daily at specified time
    pub fn daily_at(hour: u32, minute: u32) -> ValueResult<Self> {
        if hour > 23 {
            return Err(ValueError::custom("Hour must be 0-23"));
        }
        if minute > 59 {
            return Err(ValueError::custom("Minute must be 0-59"));
        }
        Self::new(format!("{} {} * * *", minute, hour))
    }

    /// Creates a cron expression that runs weekly on Sunday at midnight
    #[must_use]
    pub fn weekly() -> Self {
        Self::new("0 0 * * 0").unwrap()
    }

    /// Creates a cron expression that runs weekly on specified day and time
    pub fn weekly_at(day_of_week: u32, hour: u32, minute: u32) -> ValueResult<Self> {
        if day_of_week > 6 {
            return Err(ValueError::custom("Day of week must be 0-6 (Sunday=0)"));
        }
        if hour > 23 {
            return Err(ValueError::custom("Hour must be 0-23"));
        }
        if minute > 59 {
            return Err(ValueError::custom("Minute must be 0-59"));
        }
        Self::new(format!("{} {} * * {}", minute, hour, day_of_week))
    }

    /// Creates a cron expression that runs monthly on the 1st at midnight
    #[must_use]
    pub fn monthly() -> Self {
        Self::new("0 0 1 * *").unwrap()
    }

    /// Creates a cron expression that runs monthly on specified day and time
    pub fn monthly_at(day: u32, hour: u32, minute: u32) -> ValueResult<Self> {
        if day < 1 || day > 31 {
            return Err(ValueError::custom("Day must be 1-31"));
        }
        if hour > 23 {
            return Err(ValueError::custom("Hour must be 0-23"));
        }
        if minute > 59 {
            return Err(ValueError::custom("Minute must be 0-59"));
        }
        Self::new(format!("{} {} {} * *", minute, hour, day))
    }

    /// Creates a cron expression that runs at specified intervals
    pub fn every_n_minutes(n: u32) -> ValueResult<Self> {
        if n == 0 || n > 59 {
            return Err(ValueError::custom("Interval must be 1-59 minutes"));
        }
        Self::new(format!("*/{} * * * *", n))
    }

    /// Creates a cron expression that runs every N hours
    pub fn every_n_hours(n: u32) -> ValueResult<Self> {
        if n == 0 || n > 23 {
            return Err(ValueError::custom("Interval must be 1-23 hours"));
        }
        Self::new(format!("0 */{} * * *", n))
    }

    // === Expression Information ===

    /// Returns the cron expression string
    #[inline]
    #[must_use]
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Returns the parts of the cron expression
    #[must_use]
    pub fn parts(&self) -> CronParts {
        CronParts {
            minute: self.parsed.minute.clone(),
            hour: self.parsed.hour.clone(),
            day: self.parsed.day.clone(),
            month: self.parsed.month.clone(),
            day_of_week: self.parsed.day_of_week.clone(),
        }
    }

    /// Returns a human-readable description of the schedule
    #[must_use]
    pub fn description(&self) -> String {
        self.parsed.describe()
    }

    /// Returns whether this is a simple interval (every N minutes/hours)
    #[must_use]
    pub fn is_simple_interval(&self) -> bool {
        let parts = self.parts();

        // Check for patterns like "*/N * * * *" or "0 */N * * *"
        (parts.minute.contains('/')
            && parts.hour == "*"
            && parts.day == "*"
            && parts.month == "*"
            && parts.day_of_week == "*")
            || (parts.minute == "0"
                && parts.hour.contains('/')
                && parts.day == "*"
                && parts.month == "*"
                && parts.day_of_week == "*")
    }

    /// Returns whether this cron runs multiple times per day
    #[must_use]
    pub fn runs_multiple_times_daily(&self) -> bool {
        let parts = self.parts();
        parts.minute.contains('*')
            || parts.minute.contains('/')
            || parts.hour.contains('*')
            || parts.hour.contains('/')
    }

    // === Schedule Calculation ===

    /// Calculate the next execution time from now
    #[must_use]
    pub fn next_execution(&self) -> Option<DateTime<Utc>> {
        self.next_execution_after(Utc::now())
    }

    /// Calculate the next execution time after the given datetime
    #[must_use]
    pub fn next_execution_after(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.parsed.next_execution_after(after)
    }

    /// Calculate the previous execution time before now
    #[must_use]
    pub fn previous_execution(&self) -> Option<DateTime<Utc>> {
        self.previous_execution_before(Utc::now())
    }

    /// Calculate the previous execution time before the given datetime
    #[must_use]
    pub fn previous_execution_before(&self, before: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.parsed.previous_execution_before(before)
    }

    /// Calculate the next N execution times
    #[must_use]
    pub fn next_n_executions(&self, n: usize) -> Vec<DateTime<Utc>> {
        self.next_n_executions_after(Utc::now(), n)
    }

    /// Calculate the next N execution times after the given datetime
    #[must_use]
    pub fn next_n_executions_after(&self, after: DateTime<Utc>, n: usize) -> Vec<DateTime<Utc>> {
        let mut executions = Vec::with_capacity(n);
        let mut current = after;

        for _ in 0..n {
            if let Some(next) = self.next_execution_after(current) {
                executions.push(next);
                current = next + ChronoDuration::seconds(1); // Move past this execution
            } else {
                break;
            }
        }

        executions
    }

    /// Check if the given datetime matches this cron schedule
    #[must_use]
    pub fn matches(&self, datetime: DateTime<Utc>) -> bool {
        self.parsed.matches(datetime)
    }

    // === Time Until Next/Since Last ===

    /// Calculate time until next execution
    #[must_use]
    pub fn time_until_next(&self) -> Option<ChronoDuration> {
        self.next_execution().map(|next| next - Utc::now())
    }

    /// Calculate time since last execution
    #[must_use]
    pub fn time_since_last(&self) -> Option<ChronoDuration> {
        self.previous_execution().map(|prev| Utc::now() - prev)
    }

    // === Validation Methods ===

    /// Check if this cron expression is valid for production use
    #[must_use]
    pub fn is_production_safe(&self) -> bool {
        // Avoid expressions that run too frequently (more than once per minute)
        !self.expression.starts_with("*") && !self.runs_every_second()
    }

    /// Check if this cron runs every second (which would be excessive)
    #[must_use]
    pub fn runs_every_second(&self) -> bool {
        // This is a simple check - in practice, standard cron doesn't support seconds
        false
    }

    /// Check if the cron expression might be too frequent for workflow scheduling
    #[must_use]
    pub fn is_reasonable_frequency(&self) -> bool {
        // Allow minimum 1-minute intervals for workflow scheduling
        if self.expression.starts_with("*") {
            return false; // Every minute might be too frequent
        }

        // Check for very frequent intervals
        if let Some(interval) = self.extract_minute_interval() {
            interval >= 5 // At least every 5 minutes
        } else {
            true // Non-interval expressions are generally reasonable
        }
    }

    /// Extract the minute interval if this is a simple interval expression
    #[must_use]
    pub fn extract_minute_interval(&self) -> Option<u32> {
        let parts = self.parts();
        if parts.minute.starts_with("*/") {
            parts.minute.strip_prefix("*/")?.parse().ok()
        } else {
            None
        }
    }

    // === Utility Methods ===

    /// Convert to a more human-friendly format
    #[must_use]
    pub fn to_human_readable(&self) -> String {
        self.description()
    }

    /// Check if two cron expressions are functionally equivalent
    #[must_use]
    pub fn is_equivalent_to(&self, other: &CronValue) -> bool {
        // Simple check - could be more sophisticated
        self.expression == other.expression || self.normalize() == other.normalize()
    }

    /// Normalize the cron expression (convert equivalent forms)
    #[must_use]
    pub fn normalize(&self) -> String {
        // Convert "*/1" to "*", remove extra spaces, etc.
        self.expression.replace("*/1", "*").split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Validate that the next execution time is reasonable
    pub fn validate_next_execution(&self) -> ValueResult<()> {
        match self.next_execution() {
            Some(next) => {
                let time_until = next - Utc::now();
                if time_until > ChronoDuration::days(366) {
                    Err(ValueError::custom("Next execution is more than a year away"))
                } else if time_until < ChronoDuration::seconds(0) {
                    Err(ValueError::custom("Next execution is in the past"))
                } else {
                    Ok(())
                }
            },
            None => Err(ValueError::custom("No future execution time found")),
        }
    }
}

// === Helper Structures ===

/// Represents the parsed components of a cron expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct CronExpression {
    minute: String,      // 0-59
    hour: String,        // 0-23
    day: String,         // 1-31
    month: String,       // 1-12
    day_of_week: String, // 0-7 (0 and 7 are Sunday)
}

impl CronExpression {
    fn parse(expression: &str) -> ValueResult<Self> {
        let parts: Vec<&str> = expression.split_whitespace().collect();

        if parts.len() != 5 {
            return Err(ValueError::custom(format!(
                "Cron expression must have exactly 5 parts, got {}",
                parts.len()
            )));
        }

        let cron = Self {
            minute: parts[0].to_string(),
            hour: parts[1].to_string(),
            day: parts[2].to_string(),
            month: parts[3].to_string(),
            day_of_week: parts[4].to_string(),
        };

        cron.validate()?;
        Ok(cron)
    }

    fn validate(&self) -> ValueResult<()> {
        self.validate_field(&self.minute, "minute", 0, 59)?;
        self.validate_field(&self.hour, "hour", 0, 23)?;
        self.validate_field(&self.day, "day", 1, 31)?;
        self.validate_field(&self.month, "month", 1, 12)?;
        self.validate_field(&self.day_of_week, "day_of_week", 0, 7)?;
        Ok(())
    }

    fn validate_field(&self, field: &str, name: &str, min: u32, max: u32) -> ValueResult<()> {
        if field == "*" {
            return Ok(());
        }

        // Handle ranges (e.g., "1-5")
        if field.contains('-') {
            let parts: Vec<&str> = field.split('-').collect();
            if parts.len() != 2 {
                return Err(ValueError::custom(format!("Invalid range in {}: {}", name, field)));
            }
            let start: u32 = parts[0].parse().map_err(|_| {
                ValueError::custom(format!("Invalid range start in {}: {}", name, parts[0]))
            })?;
            let end: u32 = parts[1].parse().map_err(|_| {
                ValueError::custom(format!("Invalid range end in {}: {}", name, parts[1]))
            })?;

            if start < min || start > max || end < min || end > max || start > end {
                return Err(ValueError::custom(format!(
                    "Invalid range in {}: {}-{}",
                    name, start, end
                )));
            }
            return Ok(());
        }

        // Handle step values (e.g., "*/5", "1-10/2")
        if field.contains('/') {
            let parts: Vec<&str> = field.split('/').collect();
            if parts.len() != 2 {
                return Err(ValueError::custom(format!("Invalid step in {}: {}", name, field)));
            }

            let step: u32 = parts[1].parse().map_err(|_| {
                ValueError::custom(format!("Invalid step value in {}: {}", name, parts[1]))
            })?;

            if step == 0 {
                return Err(ValueError::custom(format!("Step value cannot be zero in {}", name)));
            }

            // Validate the base part (before the /)
            if parts[0] != "*" {
                self.validate_field(parts[0], name, min, max)?;
            }
            return Ok(());
        }

        // Handle comma-separated lists (e.g., "1,3,5")
        if field.contains(',') {
            for part in field.split(',') {
                self.validate_field(part.trim(), name, min, max)?;
            }
            return Ok(());
        }

        // Handle single values
        let value: u32 = field
            .parse()
            .map_err(|_| ValueError::custom(format!("Invalid value in {}: {}", name, field)))?;

        if value < min || value > max {
            return Err(ValueError::custom(format!(
                "Value {} out of range for {}: {}-{}",
                value, name, min, max
            )));
        }

        Ok(())
    }

    #[allow(clippy::cognitive_complexity)]
    fn describe(&self) -> String {
        // This is a simplified description generator
        let mut parts = Vec::new();

        // Minute
        match self.minute.as_str() {
            "*" => parts.push("every minute".to_string()),
            "0" => parts.push("at minute 0".to_string()),
            minute if minute.starts_with("*/") => {
                let interval = minute.strip_prefix("*/").unwrap();
                parts.push(format!("every {} minutes", interval));
            },
            minute => parts.push(format!("at minute {}", minute)),
        }

        // Hour
        match self.hour.as_str() {
            "*" => {}, // Already covered by minute
            "0" => parts.push("at midnight".to_string()),
            "12" => parts.push("at noon".to_string()),
            hour if hour.starts_with("*/") => {
                let interval = hour.strip_prefix("*/").unwrap();
                parts.push(format!("every {} hours", interval));
            },
            hour => parts.push(format!("at hour {}", hour)),
        }

        // Day of month
        match self.day.as_str() {
            "*" => {},
            "1" => parts.push("on the 1st".to_string()),
            day => parts.push(format!("on day {}", day)),
        }

        // Month
        match self.month.as_str() {
            "*" => {},
            month => {
                let month_name = match month {
                    "1" => "January",
                    "2" => "February",
                    "3" => "March",
                    "4" => "April",
                    "5" => "May",
                    "6" => "June",
                    "7" => "July",
                    "8" => "August",
                    "9" => "September",
                    "10" => "October",
                    "11" => "November",
                    "12" => "December",
                    _ => month,
                };
                parts.push(format!("in {}", month_name));
            },
        }

        // Day of week
        match self.day_of_week.as_str() {
            "*" => {},
            "0" | "7" => parts.push("on Sunday".to_string()),
            "1" => parts.push("on Monday".to_string()),
            "2" => parts.push("on Tuesday".to_string()),
            "3" => parts.push("on Wednesday".to_string()),
            "4" => parts.push("on Thursday".to_string()),
            "5" => parts.push("on Friday".to_string()),
            "6" => parts.push("on Saturday".to_string()),
            day => parts.push(format!("on day {} of week", day)),
        }

        if parts.is_empty() { "every minute".to_string() } else { parts.join(", ") }
    }

    fn next_execution_after(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        // This is a simplified implementation
        // A full implementation would need to handle all cron complexities
        let mut candidate = after.with_second(0)?.with_nanosecond(0)? + ChronoDuration::minutes(1);

        // Search for the next valid time (with a reasonable limit)
        for _ in 0..366 * 24 * 60 {
            // Search up to a year
            if self.matches(candidate) {
                return Some(candidate);
            }
            candidate += ChronoDuration::minutes(1);
        }

        None
    }

    fn previous_execution_before(&self, before: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let mut candidate = before.with_second(0)?.with_nanosecond(0)? - ChronoDuration::minutes(1);

        // Search for the previous valid time (with a reasonable limit)
        for _ in 0..366 * 24 * 60 {
            // Search up to a year back
            if self.matches(candidate) {
                return Some(candidate);
            }
            candidate -= ChronoDuration::minutes(1);
        }

        None
    }

    fn matches(&self, datetime: DateTime<Utc>) -> bool {
        self.matches_field(&self.minute, datetime.minute())
            && self.matches_field(&self.hour, datetime.hour())
            && self.matches_field(&self.day, datetime.day())
            && self.matches_field(&self.month, datetime.month())
            && self.matches_day_of_week(&self.day_of_week, datetime)
    }

    fn matches_field(&self, field: &str, value: u32) -> bool {
        if field == "*" {
            return true;
        }

        // Handle step values
        if field.contains('/') {
            let parts: Vec<&str> = field.split('/').collect();
            if parts.len() == 2 {
                if let Ok(step) = parts[1].parse::<u32>() {
                    if parts[0] == "*" {
                        return value % step == 0;
                    }
                }
            }
        }

        // Handle ranges
        if field.contains('-') {
            let parts: Vec<&str> = field.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    return value >= start && value <= end;
                }
            }
        }

        // Handle lists
        if field.contains(',') {
            return field
                .split(',')
                .any(|part| part.trim().parse::<u32>().map_or(false, |v| v == value));
        }

        // Handle single value
        field.parse::<u32>().map_or(false, |v| v == value)
    }

    fn matches_day_of_week(&self, field: &str, datetime: DateTime<Utc>) -> bool {
        if field == "*" {
            return true;
        }

        let weekday = datetime.weekday().num_days_from_sunday();

        // Handle 7 as Sunday (same as 0)
        let field_normalized = field.replace('7', "0");
        self.matches_field(&field_normalized, weekday)
    }
}

/// Public representation of cron expression parts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CronParts {
    pub minute: String,
    pub hour: String,
    pub day: String,
    pub month: String,
    pub day_of_week: String,
}

// === Trait Implementations ===

impl fmt::Display for CronValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Pretty format with description
            write!(f, "{} ({})", self.expression, self.description())
        } else {
            // Just the expression
            write!(f, "{}", self.expression)
        }
    }
}

impl FromStr for CronValue {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for CronValue {
    type Error = ValueError;

    fn try_from(expression: &str) -> Result<Self, Self::Error> {
        Self::new(expression)
    }
}

impl TryFrom<String> for CronValue {
    type Error = ValueError;

    fn try_from(expression: String) -> Result<Self, Self::Error> {
        Self::new(expression)
    }
}

// === JSON Conversion ===
#[cfg(feature = "json")]
impl From<CronValue> for serde_json::Value {
    fn from(cron: CronValue) -> Self {
        serde_json::Value::String(cron.expression)
    }
}
#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for CronValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::String(expression) => Self::new(expression),
            other => Err(ValueError::type_conversion_with_value(
                format!("{:?}", other),
                "CronValue",
                "expected string cron expression".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let cron = CronValue::new("0 0 * * *").unwrap();
        assert_eq!(cron.expression(), "0 0 * * *");

        let daily = CronValue::daily();
        assert_eq!(daily.expression(), "0 0 * * *");

        let hourly = CronValue::every_hour();
        assert_eq!(hourly.expression(), "0 * * * *");
    }

    #[test]
    fn test_validation() {
        assert!(CronValue::new("0 0 * * *").is_ok());
        assert!(CronValue::new("invalid").is_err());
        assert!(CronValue::new("60 0 * * *").is_err()); // Invalid minute

        let cron = CronValue::every_minute();
        assert!(!cron.is_production_safe()); // Too frequent

        let reasonable = CronValue::every_n_minutes(10).unwrap();
        assert!(reasonable.is_reasonable_frequency());
    }

    #[test]
    fn test_scheduling() {
        let cron = CronValue::daily_at(14, 30).unwrap();

        let next = cron.next_execution();
        assert!(next.is_some());

        let executions = cron.next_n_executions(3);
        assert_eq!(executions.len(), 3);

        // Each execution should be 24 hours apart
        if executions.len() >= 2 {
            let diff = executions[1] - executions[0];
            assert_eq!(diff, ChronoDuration::hours(24));
        }
    }

    #[test]
    fn test_description() {
        let daily = CronValue::daily();
        let desc = daily.description();
        assert!(desc.contains("midnight") || desc.contains("0"));

        let every_5min = CronValue::every_n_minutes(5).unwrap();
        let desc = every_5min.description();
        assert!(desc.contains("5 minutes"));
    }

    #[test]
    fn test_matching() {
        let cron = CronValue::new("30 14 * * *").unwrap(); // 2:30 PM daily

        // Create a datetime for 2:30 PM
        let test_time = Utc::now()
            .with_hour(14)
            .unwrap()
            .with_minute(30)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        assert!(cron.matches(test_time));

        // 2:31 PM should not match
        let test_time_31 = test_time.with_minute(31).unwrap();
        assert!(!cron.matches(test_time_31));
    }

    #[test]
    fn test_intervals() {
        let every_5 = CronValue::every_n_minutes(5).unwrap();
        assert_eq!(every_5.extract_minute_interval(), Some(5));
        assert!(every_5.is_simple_interval());

        let complex = CronValue::new("30 9-17 * * 1-5").unwrap(); // 9:30 AM, weekdays
        assert!(!complex.is_simple_interval());
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_json_conversion() {
        let cron = CronValue::daily();
        let json: serde_json::Value = cron.into();
        assert_eq!(json, serde_json::Value::String("0 0 * * *".to_string()));

        let parsed: CronValue =
            serde_json::Value::String("0 12 * * *".to_string()).try_into().unwrap();
        assert_eq!(parsed.expression(), "0 12 * * *");
    }
}
