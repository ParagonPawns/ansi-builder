pub struct ColorFg(AnsiBuilder);

impl ColorFg {
    #[inline]
    pub fn black(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[30m");
        self.0
    }

    #[inline]
    pub fn blue(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[34m");
        self.0
    }

    #[inline]
    pub fn bright_blue(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[94m");
        self.0
    }

    #[inline]
    pub fn bright_cyan(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[96m");
        self.0
    }

    #[inline]
    pub fn bright_green(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[92m");
        self.0
    }

    #[inline]
    pub fn bright_magenta(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[95m");
        self.0
    }

    #[inline]
    pub fn bright_red(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[91m");
        self.0
    }

    #[inline]
    pub fn bright_white(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[97m");
        self.0
    }

    #[inline]
    pub fn bright_yellow(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[93");
        self.0
    }

    #[inline]
    pub fn cyan(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[36m");
        self.0
    }

    #[inline]
    pub fn gray(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[90m");
        self.0
    }

    #[inline]
    pub fn green(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[32m");
        self.0
    }

    #[inline]
    pub fn magenta(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[35m");
        self.0
    }

    #[inline]
    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> AnsiBuilder {
        self.0.0.push_str(&format!("\x1B[38;2;{};{};{}", r, g, b));
        self.0
    }

    #[inline]
    pub fn red(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[31m");
        self.0
    }

    #[inline]
    pub fn white(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[37m");
        self.0
    }

    #[inline]
    pub fn yellow(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[33m");
        self.0
    }
}

pub struct ColorBg(AnsiBuilder);

impl ColorBg {
    #[inline]
    pub fn black(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[40m");
        self.0
    }

    #[inline]
    pub fn blue(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[44m");
        self.0
    }

    #[inline]
    pub fn bright_blue(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[104m");
        self.0
    }

    #[inline]
    pub fn bright_cyan(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[106m");
        self.0
    }

    #[inline]
    pub fn bright_green(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[102m");
        self.0
    }

    #[inline]
    pub fn bright_magenta(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[105m");
        self.0
    }

    #[inline]
    pub fn bright_red(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[101m");
        self.0
    }

    #[inline]
    pub fn bright_white(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[107m");
        self.0
    }

    #[inline]
    pub fn bright_yellow(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[103");
        self.0
    }

    #[inline]
    pub fn cyan(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[46m");
        self.0
    }

    #[inline]
    pub fn gray(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[100m");
        self.0
    }

    #[inline]
    pub fn green(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[42m");
        self.0
    }

    #[inline]
    pub fn magenta(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[45m");
        self.0
    }

    #[inline]
    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> AnsiBuilder {
        self.0.0.push_str(&format!("\x1B[48;2;{};{};{}", r, g, b));
        self.0
    }

    #[inline]
    pub fn red(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[41m");
        self.0
    }

    #[inline]
    pub fn white(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[47m");
        self.0
    }

    #[inline]
    pub fn yellow(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[43m");
        self.0
    }
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
