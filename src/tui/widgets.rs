// Common widgets used across screens
pub struct StatusBar {
    pub message: String,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            message: String::new(),
        }
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
}
