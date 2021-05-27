# ANSI Builder
ANSI builder is a library that allows you to build and execute ansi control
sequences. This repository is currently a work in progress and might have
frequent changes to design (will try to keep design changes to a minimum).

## Example
More details on using the library can be found in the `examples/` directory.
Also https://github.com/ParagonPawns/term-inquiry is on of our projects that
use this.

### Change color example
```rust
use ansi_builder::AnsiBuilder;

AnsiBuilder::new()
    .color().fg().red()
    .text("This text will be red")
    .print() // prints out what we currently have to the terminal.
    .reset_attributes()
    .text("this text will be default)
    .println(); // prints out what we have to the terminal and goes to next line.
```

### Cursor example
```rust
use ansi_builder::AnsiBuilder;

AnsiBuilder::new()
    .cursor().save() // saves current cursor positon
    .text("just writing stuff")
    .println()
    .text("more stuff")
    .println()
    .cursor().restore() // brings cursor where we saved
    .erase_display(EraseMode::Everything)
    .print();
```
