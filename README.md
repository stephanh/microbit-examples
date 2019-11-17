# Microbit example apps in Rust

A set of apps for the [microbit](https://microbit.org/) to experiment with
embedded programming in Rust.

See the readme of the individual apps for more details.

## Usage

1. Connect the Microbit via USB.
1. Run openocd `openocd -f interface/cmsis-dap.cfg -f target/nrf51.cfg` or equivalent.
1. Flash and debug one of the apps `cargo run <app>`
