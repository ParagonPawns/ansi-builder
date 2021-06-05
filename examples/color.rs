fn main() {
    #[cfg(windows)]
    if !ansi_builder::enable_ansi_color() {
        println!("Failed to enable_ansi_color.");
    }

    AnsiBuilder::new()
        .text("Hello, World! ")
        .color().fg().red()
        .text("Hello, World but in red!")
        .println()
        .color().fg().bright_blue()
        .color().bg().red()
        .text("Another sample text")
        .reset_attributes()
        .text(" Everything is reverted")
        .println();
}

use ansi_builder::AnsiBuilder;
