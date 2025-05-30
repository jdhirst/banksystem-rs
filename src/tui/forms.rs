use std::sync::Arc;
use tui::style::{Style, Color};
use tui::widgets::{Block, Borders, Paragraph};
use tui::text::{Spans, Span};
use tui::layout::Rect;

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

impl PartialEq for FormField {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label &&
        self.value == other.value &&
        self.required == other.required &&
        self.max_length == other.max_length
        // Skip comparing validation function
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

    pub fn render(&self, focused: bool) -> Spans {
        let style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Green)
        };

        let required_mark = if self.required { "*" } else { " " };
        let field_content = format!("{}{}: {}_", required_mark, self.label, self.value);

        Spans::from(vec![
            Span::styled(field_content, style)
        ])
    }
}

pub struct Form {
    pub fields: Vec<FormField>,
    pub focused_field: usize,
    title: String,
}

impl Clone for Form {
    fn clone(&self) -> Self {
        Self {
            fields: self.fields.clone(),
            focused_field: self.focused_field,
            title: self.title.clone(),
        }
    }
}

impl PartialEq for Form {
    fn eq(&self, other: &Self) -> bool {
        self.fields == other.fields &&
        self.focused_field == other.focused_field &&
        self.title == other.title
    }
}

impl Form {
    pub fn new(title: &str, fields: Vec<FormField>) -> Self {
        Self {
            fields,
            focused_field: 0,
            title: title.to_string(),
        }
    }

    pub fn render<B: tui::backend::Backend>(
        &self,
        frame: &mut tui::Frame<B>,
        area: Rect,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.as_str());

        let mut content = vec![];
        for (idx, field) in self.fields.iter().enumerate() {
            content.push(field.render(idx == self.focused_field));
        }

        let help_text = Spans::from(vec![
            Span::styled(
                "\nTAB=Next field  ENTER=Submit  ESC=Cancel",
                Style::default().fg(Color::Green),
            )
        ]);
        content.push(help_text);

        let paragraph = Paragraph::new(content)
            .block(block)
            .style(Style::default().fg(Color::Green));

        frame.render_widget(paragraph, area);
    }

    pub fn next_field(&mut self) {
        self.focused_field = (self.focused_field + 1) % self.fields.len();
    }

    pub fn prev_field(&mut self) {
        if self.focused_field == 0 {
            self.focused_field = self.fields.len() - 1;
        } else {
            self.focused_field -= 1;
        }
    }

    pub fn input(&mut self, c: char) {
        let field = &mut self.fields[self.focused_field];
        if field.value.len() < field.max_length {
            field.value.push(c);
        }
    }

    pub fn backspace(&mut self) {
        let field = &mut self.fields[self.focused_field];
        field.value.pop();
    }

    pub fn is_valid(&self) -> bool {
        self.fields.iter().all(|f| f.is_valid())
    }

    pub fn get_values(&self) -> Vec<String> {
        self.fields.iter().map(|f| f.value.clone()).collect()
    }
}
