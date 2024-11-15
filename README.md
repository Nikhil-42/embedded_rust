# Video Square

This project controls a 8x8 Neomatrix LED display using the RP2040 Feather and the Prop-Maker feather.

## Contents
Python:
- requirements.txt - Python requirements for the video_to_raw.py and video_viewer scripts
- video_to_raw.py - Python script to convert a video file to bytes for the RP2040
- video_viewer.py - Python script to preview the raw file on the computer

Rust:
- Cargo.toml - Rust project configuration
- src - Firmware source code for the RP2040
- .cargo - Cargo build/run configuration for the RP2040
- build.rs - Build script for facilitating linking against the memory.x file
- memory.x - Memory layout for the RP2040 for the linker

This project is developed and maintained by the [Cortex-M team][team].

## Dependencies

To build embedded programs using this template you'll need:

- The elf2uf2 tool. You can install it by running:

``` console
$ cargo install elf2uf2-rs
```

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M0+
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi
```

## Running the project

To run the project:
1. connect the RP2040 Feather to your computer using a USB cable. 
2. Hold the BOOTSEL button on the RP2040 Feather and click the RESET button.
3. Release the BOOTSEL button.
4. The RP2040 Feather should now be in bootloader mode and you should see a new drive appear on your computer.
5. Mount the new drive.
6. Finally, run ```console $ cargo run``` to flash the firmware to the RP2040 Feather.
