fn main() {
    #[cfg(windows)]
    windows::build! {
        Windows::Win32::System::Console::{
            CONSOLE_MODE,
            ENABLE_PROCESSED_OUTPUT,
            ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            GetConsoleMode,
            SetConsoleMode
        },
        Windows::Win32::System::SystemServices::{
            HANDLE
        }
    };
}
