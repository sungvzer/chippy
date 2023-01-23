# chippy 

A Chip-8 emulator written in Rust

<p align="center">
    <img width="600" alt="immagine" src="https://user-images.githubusercontent.com/52582911/214082817-5f123b6c-6f4e-4a77-afa4-957677637fb8.png"/>
    <p align="center">
     Chippy running a demo from Marco Varesio
    </p>
</p>

## About

Chip-8 is an interpreted programming language developed initially for the COSMAC VIP microcomputer. It has a total of 35 opcodes, which makes it easy to implement an emulator to run CHIP-8 code.

[Read more on Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)

## Documentation

Some documentation about the project is available in the [docs/spec.md](./docs/spec.md) file; it is rewritten based on an existing 1997 reference manual I found on the internet.

## Implementation

### Emulation

All the emulation is done by software, and is written without the usage of external libraries.

### Graphics

The screen rendering is done via [pixels](https://docs.rs/pixels/latest/pixels/) - for the 2D pixel rendering - and [tao](https://docs.rs/tao/latest/tao/) for the window management and event loop.

### Audio

As the programming language only provides a single-frequency tone to be played, sound is handled via [cpal](https://docs.rs/cpal/latest/cpal/).
