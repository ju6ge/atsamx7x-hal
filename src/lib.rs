#![no_std]
#![feature(core_intrinsics)]

extern crate embedded_hal as hal;
extern crate nb;

pub extern crate atsame70q21  as target_device;

pub mod gpio;
pub mod serial;
pub mod time;
pub mod delay;
pub mod clock_gen;
