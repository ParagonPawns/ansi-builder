pub mod color;
pub mod cursor;
pub mod style;

pub enum EraseMode {
    CursorToEnd = 0,
    CursorToBegin = 1,
    Screen = 2,
    Everything = 3,
}

pub enum ClearMode {
    CursorToEnd = 0,
    CursorToBegin = 1,
    EntireLine = 2,
}

pub struct AnsiBuilder(String);

impl AnsiBuilder {
    pub fn new() -> Self {
        let val = String::with_capacity(40);
        Self(val)
    }

    pub fn erase_line(mut self, mode: ClearMode) -> Self {
        self.0.push_str(&format!("\x1B[{}K", mode as u8));
        self
    }

    pub fn cursor(self) -> Cursor {
        Cursor(self)
    }

    pub fn color(self) -> Color {
        Color(self)
    }

    pub fn text(mut self, string: &str) -> Self {
        self.0.push_str(string);
        self
    }

    pub fn erase_in_display(mut self, mode: EraseMode) -> Self {
        self.0.push_str(&format!("\x1B[{}J", mode as u8));
        self
    }

    pub fn print(mut self) -> Self {
        print!("{}", self.0);
        self.0.clear();
        self
    }

    pub fn println(mut self) -> Self {
        println!("{}", self.0);
        self.0.clear();
        self
    }

    pub fn reset_attributes(mut self) -> Self {
        self.0.push_str("\x1B[0m");
        self
    }

    pub fn style(self) -> Style {
        Style(self)
    }
}

use color::Color;
use cursor::Cursor;
use style::Style;
