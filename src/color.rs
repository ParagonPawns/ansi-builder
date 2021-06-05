pub struct ColorFg(AnsiBuilder);

impl ColorFg {
    ansi_sgr!(black, "30");
    ansi_sgr!(blue, "34");
    ansi_sgr!(bright_blue, "94");
    ansi_sgr!(bright_cyan, "96");
    ansi_sgr!(bright_green, "92");
    ansi_sgr!(bright_magenta, "95");
    ansi_sgr!(bright_red, "91");
    ansi_sgr!(bright_white, "97");
    ansi_sgr!(bright_yellow, "93");
    ansi_sgr!(cyan, "36");
    ansi_sgr!(gray, "90");
    ansi_sgr!(green, "32");
    ansi_sgr!(magenta, "35");
    ansi_sgr!(red, "31");
    ansi_sgr!(white, "37");
    ansi_sgr!(yellow, "33");

    #[inline]
    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> AnsiBuilder {
        self.0.0.push_str(&format!("\x1B[38;2;{};{};{}", r, g, b));
        self.0
    }
}

pub struct ColorBg(AnsiBuilder);

impl ColorBg {
    ansi_sgr!(black, "40");
    ansi_sgr!(blue, "44");
    ansi_sgr!(bright_blue, "104");
    ansi_sgr!(bright_cyan, "106");
    ansi_sgr!(bright_green, "102");
    ansi_sgr!(bright_magenta, "105");
    ansi_sgr!(bright_red, "101");
    ansi_sgr!(bright_white, "107");
    ansi_sgr!(bright_yellow, "103");
    ansi_sgr!(cyan, "46");
    ansi_sgr!(gray, "100");
    ansi_sgr!(green, "42");
    ansi_sgr!(magenta, "45");
    ansi_sgr!(red, "41");
    ansi_sgr!(white, "47");
    ansi_sgr!(yellow, "43");

    #[inline]
    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> AnsiBuilder {
        self.0.0.push_str(&format!("\x1B[48;2;{};{};{}", r, g, b));
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
