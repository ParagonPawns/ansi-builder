/// Style struct takes in the AnsiBuilder and set the correct escape sequence
/// to fit the wanted style that is asked for in future text.
///
/// ## Example
/// ```
/// use ansi_builder::AnsiBuilder;
///
/// // "Hello" will be bold in the terminal.
/// AnsiBuilder::new()
///     .style().bold()
///     .text("Hello")
///     .println();
/// ```
pub struct Style(pub AnsiBuilder);

impl Style {
    ansi_sgr!(bold, "1");
    ansi_sgr!(conceal, "8");
    ansi_sgr!(double_underline,"21");
    ansi_sgr!(dim, "2");
    ansi_sgr!(encircle, "52");
    ansi_sgr!(frame, "51");
    ansi_sgr!(italic, "3");
    ansi_sgr!(normal, "25");
    ansi_sgr!(overline, "53");
    ansi_sgr!(rapid_blink, "6");
    ansi_sgr!(remove_blink, "25");
    ansi_sgr!(remove_frame_and_encircle, "54");
    ansi_sgr!(remove_italic, "23");
    ansi_sgr!(remove_overline, "55");
    ansi_sgr!(remove_strike, "29");
    ansi_sgr!(remove_underline, "24");
    ansi_sgr!(reveal, "28");
    ansi_sgr!(slow_blink, "5");
    ansi_sgr!(strike, "9");
    ansi_sgr!(underline, "4");
}

use crate::AnsiBuilder;
