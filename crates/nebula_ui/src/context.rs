use crate::theme::Theme;
use std::collections::HashMap;

pub struct RenderContext {
    pub theme: Theme,
    pub validation_errors: HashMap<String, Vec<String>>,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            validation_errors: HashMap::new(),
        }
    }
}

impl RenderContext {
    pub fn new(theme: Theme) -> Self {
        Self {
            theme,
            validation_errors: HashMap::new(),
        }
    }

    pub fn add_validation_error(&mut self, key: String, error: String) {
        self.validation_errors
            .entry(key)
            .or_insert_with(Vec::new)
            .push(error);
    }

    pub fn clear_validation_errors(&mut self, key: &str) {
        self.validation_errors.remove(key);
    }

    pub fn has_errors(&self, key: &str) -> bool {
        self.validation_errors
            .get(key)
            .map(|errors| !errors.is_empty())
            .unwrap_or(false)
    }

    pub fn get_errors(&self, key: &str) -> Option<&Vec<String>> {
        self.validation_errors.get(key)
    }
}