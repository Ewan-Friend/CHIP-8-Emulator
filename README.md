# CHIP-8 Emulator

A CHIP-8 emulator written in Rust, built from Cowgod's CHIP-8 technical reference. The project implements the full virtual machine (memory, registers, stack, timers, keypad, display and instruction set) and runs classic CHIP-8 programs on the desktop.

## Overview

CHIP-8 is an interpreted language from the 1970s, designed to run games on small 8-bit microcomputers. Emulating it is a compact way to work through the core ideas behind real processors: the fetch-decode-execute cycle, memory-mapped state, opcode decoding, timing, and input/output handling. This project was built to deepen my understanding of those concepts after studying computer architecture at university.

## Design

The repository is split into two parts:

- `chip8_core`: the emulator itself, kept separate from any platform-specific code. It handles instruction decoding and execution, the 4KB memory space, registers, the call stack, delay and sound timers, the keypad state and the display buffer.
- `desktop`: a frontend that drives the core, renders the display, and maps keyboard input to the CHIP-8 keypad.
- `roms`: sample programs for testing.

Separating the core from the frontend keeps the emulation logic independent of how it is displayed or controlled, which makes it easier to test and to reuse with a different frontend.

## What I learned

- Translating a hardware specification into working software, with the manual as the only reference.
- Structuring a Rust project as a workspace with a reusable library crate and a separate application.
- Working with low-level concepts: bitwise operations, opcode decoding, and fixed-size memory and registers.
- Frontend work (rendering, input and timing) turned out to be the hardest part, because it meant keeping the display, input and timers in step with the emulated CPU.

## Running

Requires a recent stable Rust toolchain (install via https://rustup.rs).

Clone the repository and run the desktop frontend with a ROM from the `roms` folder:

    git clone https://github.com/Ewan-Friend/CHIP-8-Emulator.git
    cd CHIP-8-Emulator/desktop
    cargo run --release -- ../roms/<rom-name>.ch8

To build without running:

    cargo build --release

## Controls

The original CHIP-8 keypad is a 4x4 hex grid. It is mapped onto the left side of a QWERTY keyboard:

    CHIP-8 keypad        Keyboard

    1  2  3  C           1  2  3  4
    4  5  6  D           Q  W  E  R
    7  8  9  E           A  S  D  F
    A  0  B  F           Z  X  C  V

Individual games use different keys, so the keys that matter vary from ROM to ROM.

## References

- Cowgod's CHIP-8 Technical Reference
