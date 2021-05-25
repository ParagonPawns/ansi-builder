pub struct Cursor(pub AnsiBuilder);

impl Cursor {
    pub fn down(mut self, pos: usize) -> AnsiBuilder {
        self.0.0.push_str(&format!("\x1B[{}B", pos));
        self.0
    }

    pub fn hide(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[?25l");
        self.0
    }

    pub fn left(mut self, pos: usize) -> AnsiBuilder {
        self.0.0.push_str(&format!("\x1B[{}D", pos));
        self.0
    }

    pub fn restore(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[u");
        self.0
    }

    pub fn right(mut self, pos: usize) -> AnsiBuilder {
        self.0.0.push_str(&format!("\x1B[{}C", pos));
        self.0
    }

    pub fn save(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[s");
        self.0
    }

    pub fn show(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[?25h");
        self.0
    }

    pub fn up(mut self, pos: usize) -> AnsiBuilder {
        self.0.0.push_str(&format!("\x1B[{}A", pos));
        self.0
    }
}

use crate::AnsiBuilder;
