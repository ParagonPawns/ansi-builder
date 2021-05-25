pub struct Style(pub AnsiBuilder);

impl Style {
    pub fn bold(mut self) -> AnsiBuilder {
        self.0.0.push_str("\x1B[1m");
        self.0
    }
}

use crate::AnsiBuilder;
