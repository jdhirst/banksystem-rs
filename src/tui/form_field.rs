use std::sync::Arc;

pub struct FormField {
    pub label: String,
    pub value: String,
    pub required: bool,
    pub max_length: usize,
    pub validation: Arc<dyn Fn(&str) -> bool + Send + Sync>,
}

impl Clone for FormField {
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            value: self.value.clone(),
            required: self.required,
            max_length: self.max_length,
            validation: Arc::clone(&self.validation),
        }
    }
}

impl FormField {
    pub fn new(label: &str, required: bool, max_length: usize) -> Self {
        Self {
            label: label.to_string(),
            value: String::new(),
            required,
            max_length,
            validation: Arc::new(|_| true),
        }
    }

    pub fn with_validation(mut self, validation: Arc<dyn Fn(&str) -> bool + Send + Sync>) -> Self {
        self.validation = validation;
        self
    }

    pub fn is_valid(&self) -> bool {
        if self.required && self.value.is_empty() {
            return false;
        }
        (self.validation)(&self.value)
    }
}
