// src/value/comparison.rs

use super::{Value, ValueError};
use regex::Regex;

/// Utility методы для сравнения и проверки значений Value
pub struct ValueComparison;

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonResult {
    True,
    False,
    Error(ValueError),
}

impl ComparisonResult {
    pub fn is_true(&self) -> bool {
        matches!(self, Self::True)
    }

    pub fn is_false(&self) -> bool {
        matches!(self, Self::False)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    pub fn unwrap_or_false(self) -> bool {
        matches!(self, Self::True)
    }

    pub fn unwrap_or_true(self) -> bool {
        !matches!(self, Self::False)
    }
}

impl ValueComparison {
    /// Проверяет равенство значений
    pub fn equals(left: &Value, right: &Value) -> bool {
        left == right
    }

    /// Проверяет неравенство значений
    pub fn not_equals(left: &Value, right: &Value) -> bool {
        left != right
    }

    /// Проверяет, что левое значение больше правого
    pub fn greater_than(left: &Value, right: &Value) -> ComparisonResult {
        match (left.try_as_number(), right.try_as_number()) {
            (Ok(a), Ok(b)) => {
                if a > b {
                    ComparisonResult::True
                } else {
                    ComparisonResult::False
                }
            }
            (Err(e), _) | (_, Err(e)) => ComparisonResult::Error(e),
        }
    }

    pub fn less_than(left: &Value, right: &Value) -> ComparisonResult {
        match (left.try_as_number(), right.try_as_number()) {
            (Ok(a), Ok(b)) => {
                if a < b {
                    ComparisonResult::True
                } else {
                    ComparisonResult::False
                }
            }
            (Err(e), _) | (_, Err(e)) => ComparisonResult::Error(e),
        }
    }

    pub fn greater_than_or_equal(left: &Value, right: &Value) -> ComparisonResult {
        match (left.try_as_number(), right.try_as_number()) {
            (Ok(a), Ok(b)) => {
                if a >= b {
                    ComparisonResult::True
                } else {
                    ComparisonResult::False
                }
            }
            (Err(e), _) | (_, Err(e)) => ComparisonResult::Error(e),
        }
    }

    pub fn less_than_or_equal(left: &Value, right: &Value) -> ComparisonResult {
        match (left.try_as_number(), right.try_as_number()) {
            (Ok(a), Ok(b)) => {
                if a <= b {
                    ComparisonResult::True
                } else {
                    ComparisonResult::False
                }
            }
            (Err(e), _) | (_, Err(e)) => ComparisonResult::Error(e),
        }
    }

    /// Проверяет, что значение входит в список
    pub fn in_list(value: &Value, list: &[Value]) -> bool {
        list.contains(value)
    }

    /// Проверяет, что значение НЕ входит в список
    pub fn not_in_list(value: &Value, list: &[Value]) -> bool {
        !list.contains(value)
    }

    /// Проверяет, что значение пустое
    pub fn is_empty(value: &Value) -> bool {
        match value {
            Value::String(s) if s.as_ref().is_empty() => true,
            Value::Array(a) if a.is_empty() => true,
            Value::Object(o) if o.is_empty() => true,
            Value::Group(g) if g.is_empty() => true,
            Value::Null => true,
            _ => false,
        }
    }

    /// Проверяет, что значение НЕ пустое
    pub fn is_not_empty(value: &Value) -> bool {
        !Self::is_empty(value)
    }

    /// Проверяет, что строка содержит подстроку
    pub fn contains(value: &Value, substring: &Value) -> ComparisonResult {
        match (value.try_as_string(), substring.try_as_string()) {
            (Ok(text), Ok(sub)) => {
                if text.contains(sub) {
                    ComparisonResult::True
                } else {
                    ComparisonResult::False
                }
            }
            (Err(e), _) | (_, Err(e)) => ComparisonResult::Error(e),
        }
    }

    /// Проверяет, что строка начинается с префикса
    pub fn starts_with(value: &Value, prefix: &Value) -> ComparisonResult {
        match (value.try_as_string(), prefix.try_as_string()) {
            (Ok(text), Ok(pref)) => {
                if text.starts_with(pref) {
                    ComparisonResult::True
                } else {
                    ComparisonResult::False
                }
            }
            (Err(e), _) | (_, Err(e)) => ComparisonResult::Error(e),
        }
    }

    /// Проверяет, что строка заканчивается суффиксом
    pub fn ends_with(value: &Value, suffix: &Value) -> ComparisonResult {
        match (value.try_as_string(), suffix.try_as_string()) {
            (Ok(text), Ok(suff)) => {
                if text.ends_with(suff) {
                    ComparisonResult::True
                } else {
                    ComparisonResult::False
                }
            }
            (Err(e), _) | (_, Err(e)) => ComparisonResult::Error(e),
        }
    }

    /// Проверяет соответствие регулярному выражению
    pub fn matches_regex(value: &Value, pattern: &str) -> ComparisonResult {
        match value.try_as_string() {
            Ok(text) => match Regex::new(pattern) {
                Ok(regex) => {
                    if regex.is_match(text) {
                        ComparisonResult::True
                    } else {
                        ComparisonResult::False
                    }
                }
                Err(e) => {
                    ComparisonResult::Error(ValueError::invalid_regex(pattern, e.to_string()))
                }
            },
            Err(e) => ComparisonResult::Error(e),
        }
    }

    /// Проверяет минимальную длину строки
    pub fn min_length(value: &Value, min: usize) -> ComparisonResult {
        match value.try_as_string() {
            Ok(text) => {
                if text.len() >= min {
                    ComparisonResult::True
                } else {
                    ComparisonResult::False
                }
            }
            Err(e) => ComparisonResult::Error(e),
        }
    }

    /// Проверяет максимальную длину строки
    pub fn max_length(value: &Value, max: usize) -> ComparisonResult {
        match value.try_as_string() {
            Ok(text) => {
                if text.len() <= max {
                    ComparisonResult::True
                } else {
                    ComparisonResult::False
                }
            }
            Err(e) => ComparisonResult::Error(e),
        }
    }

    /// Проверяет, что числовое значение находится в диапазоне
    pub fn between(value: &Value, min: &Value, max: &Value) -> ComparisonResult {
        match (
            value.try_as_number(),
            min.try_as_number(),
            max.try_as_number(),
        ) {
            (Ok(val), Ok(min_val), Ok(max_val)) => {
                if val >= min_val && val <= max_val {
                    ComparisonResult::True
                } else {
                    ComparisonResult::False
                }
            }
            (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => ComparisonResult::Error(e),
        }
    }

    /// Проверяет точное соответствие типа и значения
    pub fn strict_equals(left: &Value, right: &Value) -> bool {
        // Более строгое сравнение, учитывающее типы
        match (left, right) {
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            (Value::Null, Value::Null) => true,
            _ => false, // Разные типы не равны
        }
    }

    /// Форматирует значение для отображения
    pub fn format_for_display(value: &Value) -> String {
        value.to_string()
    }
}

// Convenience методы для простых случаев
impl ValueComparison {
    /// Простая проверка больше чем (возвращает false при ошибке)
    pub fn gt_simple(left: &Value, right: &Value) -> bool {
        Self::greater_than(left, right).unwrap_or_false()
    }

    /// Простая проверка меньше чем (возвращает false при ошибке)
    pub fn lt_simple(left: &Value, right: &Value) -> bool {
        Self::less_than(left, right).unwrap_or_false()
    }

    /// Простая проверка больше или равно (возвращает false при ошибке)
    pub fn gte_simple(left: &Value, right: &Value) -> bool {
        Self::greater_than_or_equal(left, right).unwrap_or_false()
    }

    /// Простая проверка меньше или равно (возвращает false при ошибке)
    pub fn lte_simple(left: &Value, right: &Value) -> bool {
        Self::less_than_or_equal(left, right).unwrap_or_false()
    }

    /// Простая проверка regex (возвращает false при ошибке)
    pub fn regex_simple(value: &Value, pattern: &str) -> bool {
        Self::matches_regex(value, pattern).unwrap_or_false()
    }

    /// Простая проверка contains (возвращает false при ошибке)
    pub fn contains_simple(value: &Value, substring: &Value) -> bool {
        Self::contains(value, substring).unwrap_or_false()
    }
}
