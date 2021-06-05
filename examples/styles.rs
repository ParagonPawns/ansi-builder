fn main() {
    AnsiBuilder::new()
        .style().underline()
        .text("Underlined")
        .style().remove_underline()
        .println()

        .style().italic()
        .text("italic")
        .style().remove_italic()
        .println()

        .style().dim()
        .text("dim text ")
        .reset_attributes()
        .text("not dim")
        .println()

        .style().double_underline()
        .text("double_underline")
        .style().remove_underline()
        .println()

        .style().strike()
        .text("striked text")
        .style().remove_strike()
        .println()

        .style().rapid_blink()
        .text("rapid bliking text (not widely supported) ")
        .style().remove_blink()
        .println()

        .style().slow_blink()
        .text("slow blink text")
        .style().remove_blink()
        .println()

        .style().conceal()
        .text("Hidden text")
        .style().reveal()
        .println()

        .style().strike()
        .style().double_underline()
        .text("multi-style")
        .reset_attributes()
        .println()


        .style().frame()
        .text("framed text")
        .style().remove_frame_and_encircle()
        .println()

        .style().overline()
        .text("Overlined text")
        .style().remove_overline()
        .println()

        .style().encircle()
        .text("encircle")
        .style().remove_frame_and_encircle()
        .println()

        .reset_attributes()
        .print();
}

use ansi_builder::AnsiBuilder;
