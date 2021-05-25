pub struct ColorFg(AnsiBuilder);

impl ColorFg {
    pub fn bright_blue(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[94m");
        self.0
    }

    pub fn bright_green(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[92m");
        self.0
    }

    pub fn blue(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[94m");
        self.0
    }

    pub fn gray(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[90m");
        self.0
    }

    pub fn green(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[32m");
        self.0
    }

    pub fn white(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[37m");
        self.0
    }
}

pub struct ColorBg(AnsiBuilder);

impl ColorBg {
}

pub struct Color(pub AnsiBuilder);

impl Color {
    pub fn fg(self) -> ColorFg {
        ColorFg(self.0)
    }

    pub fn bg(self) -> ColorBg {
        ColorBg(self.0)
    }
}

use crate::AnsiBuilder;
