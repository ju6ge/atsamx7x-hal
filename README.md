# ATSAMx7x Hardware Abstraction Layer

This crate implementats traits from [embedded-hal] (https://crates.io/crates/embedded-hal) for the Atmel SAMEX7X architechture.

It is still in the early stages of development, and is currently geared to run on an atsme70q21 cpu. But contributions are welcome.

# Dependencies

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```
# What works
- [x] Clock initialisation
- [x] Delays
- [x] GPIO 
- [x] UART

# Todo
- [ ] SPI
- [ ] I2C
- [ ] DMA
- [ ] Watchdog
- [ ] all other peripherals

# OpenOCD

The openocd configuration provided works for my board which is not an off the shelf board. So don't expected to just work for your platform.

