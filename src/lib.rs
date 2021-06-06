macro_rules! ansi_sgr {
    ($func:ident, $value:expr) => {
        #[inline]
        pub fn $func(mut self) -> AnsiBuilder {
            self.0.0.push_str(concat!("\x1B[",$value, "m"));
            self.0
        }
    };
}

pub mod color;
pub mod cursor;
pub mod style;

#[cfg(windows)]
::windows::include_bindings!();

#[cfg(windows)]
pub fn enable_ansi_color() -> bool {
    use std::os::windows::io::AsRawHandle;
    use std::io::stdout;
    use Windows::Win32::System::SystemServices::HANDLE;
    use Windows::Win32::System::Console::{
        CONSOLE_MODE,
        ENABLE_PROCESSED_OUTPUT,
        ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        GetConsoleMode,
        SetConsoleMode,
    };

    let std_out_handle = HANDLE(stdout().as_raw_handle() as isize);
    unsafe {
        let mut console_mode = CONSOLE_MODE::default();
        if !GetConsoleMode(std_out_handle, &mut console_mode).as_bool() {
            return false
        }

        let is_ansi_enabled =
            (console_mode & ENABLE_PROCESSED_OUTPUT).0 != 0 &&
            (console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING).0 != 0;

        if is_ansi_enabled {
            return true
        }

        if !SetConsoleMode(
            std_out_handle,
            console_mode | ENABLE_PROCESSED_OUTPUT | ENABLE_VIRTUAL_TERMINAL_PROCESSING
        ).as_bool() {
            return false
        }

        true
    }
}


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
