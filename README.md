# chippy 

A WIP Chip-8 emulator written in Rust

## About

Chip-8 is an interpreted programming language developed initially for the COSMAC VIP microcomputer. It has a total of 35 opcodes, which makes it easy to implement an emulator to run CHIP-8 code.

[Read more on Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)

## Implementation

### Emulation

All the emulation is done by software, and is written without the usage of external libraries.

### Graphics

The screen rendering is done via [pixels](https://docs.rs/pixels/latest/pixels/) - for the 2D pixel rendering - and [tao](https://docs.rs/tao/latest/tao/) for the window management and event loop.

### Audio

As the programming language only provides a single-frequency tone to be played, sound is handled via [cpal](https://docs.rs/cpal/latest/cpal/).

## Known bugs

There are currently some known bugs which make the emulator not functional at 100%:

1. Screen drawing is buggy whenever sprites are overlapping with each other. See [#2](https://github.com/sungvzer/chippy/issues/2/).
