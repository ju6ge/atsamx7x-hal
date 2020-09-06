//! Time units

/// Bits per second
#[derive(Clone, Copy, Debug)]
pub struct Bps(pub u32);

/// Hertz
#[derive(Clone, Copy, Debug)]
pub struct Hertz(pub u32);

/// KiloHertz
#[derive(Clone, Copy, Debug)]
pub struct KiloHertz(pub u32);

/// MegaHertz
#[derive(Clone, Copy, Debug)]
pub struct MegaHertz(pub u32);

/// MilliSeconds
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct MilliSeconds(pub u32);

/// MicroSeconds
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct MicroSeconds(pub u32);

/// NanoSeconds
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct NanoSeconds(pub u32);

/// PicoSeconds
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct PicoSeconds(pub u32);


/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
	/// Wrap in `Bps`
	fn bps(self) -> Bps;

	/// Wrap in `Hertz`
	fn hz(self) -> Hertz;

	/// Wrap in `KiloHertz`
	fn khz(self) -> KiloHertz;

	/// Wrap in `MegaHertz`
	fn mhz(self) -> MegaHertz;

	/// Wrap in "MilliSeconds"
	fn ms(self) -> MilliSeconds;

	/// Wrap in "MicroSeconds"
	fn us(self) -> MicroSeconds;

	/// Wrap in "NanoSeconds"
	fn ns(self) -> NanoSeconds;

	/// Wrap in "PicoSeconds"
	fn ps(self) -> PicoSeconds;
}

impl U32Ext for u32 {
	fn bps(self) -> Bps {
		Bps(self)
	}

	fn hz(self) -> Hertz {
		Hertz(self)
	}

	fn khz(self) -> KiloHertz {
		KiloHertz(self)
	}

	fn mhz(self) -> MegaHertz {
		MegaHertz(self)
	}

	fn ms(self) -> MilliSeconds {
		MilliSeconds(self)
	}

	fn us(self) -> MicroSeconds {
		MicroSeconds(self)
	}

	fn ns(self) -> NanoSeconds {
		NanoSeconds(self)
	}

	fn ps(self) -> PicoSeconds {
		PicoSeconds(self)
	}
}

// Unit conversions
impl Into<Hertz> for KiloHertz {
	fn into(self) -> Hertz {
		Hertz(self.0 * 1_000)
	}
}

impl Into<Hertz> for MegaHertz {
	fn into(self) -> Hertz {
		Hertz(self.0 * 1_000_000)
	}
}

impl Into<KiloHertz> for MegaHertz {
	fn into(self) -> KiloHertz {
		KiloHertz(self.0 * 1_000)
	}
}

// MilliSeconds <-> Hertz
impl Into<MilliSeconds> for Hertz {
	fn into(self) -> MilliSeconds {
		let freq = self.0;
		assert!(freq != 0 && freq <= 1_000);
		MilliSeconds(1_000 / freq)
	}
}


impl Into<Hertz> for MilliSeconds {
	fn into(self) -> Hertz {
		let period = self.0;
		assert!(period != 0 && period <= 1_000);
		Hertz(1_000 / period)
	}
}

impl Into<NanoSeconds> for Hertz {
	fn into(self) -> NanoSeconds{
		let freq = self.0;
		assert!(freq != 0 && freq <= 1_000_000_000);
		NanoSeconds(1_000_000_000 / freq)
	}
}

impl Into<Hertz> for NanoSeconds {
	fn into(self) -> Hertz{
		let period = self.0;
		assert!(period != 0 && period <= 1_000_000_000);
		Hertz(1_000_000_000 / period)
	}
}

impl Into<PicoSeconds> for Hertz {
	fn into(self) -> PicoSeconds{
		let freq = self.0;
		assert!(freq != 0);
		PicoSeconds((1_000_000_000_000 / freq as u64) as u32)
	}
}

impl Into<Hertz> for PicoSeconds {
	fn into(self) -> Hertz{
		let period = self.0;
		assert!(period != 0);
		Hertz((1_000_000_000_000 / period as u64) as u32)
	}
}

impl Into<PicoSeconds> for NanoSeconds {
	fn into(self) -> PicoSeconds{
		let period = self.0;
		PicoSeconds(period * 1_000)
	}
}

impl Into<PicoSeconds> for MicroSeconds {
	fn into(self) -> PicoSeconds{
		let period = self.0;
		PicoSeconds(period * 1_000_000)
	}
}


impl PicoSeconds {
	pub fn cycles(self, time:PicoSeconds) -> u32{
		let period = self.0;
		let target = time.0;
		// if target time is smaller than period, return 1 since we can't get more accurate than one cycle
		if period > target {
			return 1
		}
		let x = target / period;
		let error_x = target - period*x;
		// check if error is within one nano second, since we don't want to undershoot the cycle wait time by more
		if error_x < 1_000 {
			x
		} else {
			x+1
		}
	}
}

// use cortex_m::peripheral::DWT;
// /// A monotonic nondecreasing timer
// // #[derive(Clone, Copy)]
// // pub struct MonoTimer {
// //     frequency: Hertz,
// // }
// 
// // impl MonoTimer {
// //     /// Creates a new `Monotonic` timer
// //     pub fn new(mut dwt: DWT, clocks: Clocks) -> Self {
// //         dwt.enable_cycle_counter();
// 
// //         // now the CYCCNT counter can't be stopped or resetted
// //         drop(dwt);
// 
// //         MonoTimer {
// //             frequency: clocks.sysclk(),
// //         }
// //     }
// 
// //     /// Returns the frequency at which the monotonic timer is operating at
// //     pub fn frequency(&self) -> Hertz {
// //         self.frequency
// //     }
// 
// //     /// Returns an `Instant` corresponding to "now"
// //     pub fn now(&self) -> Instant {
// //         Instant {
// //             now: DWT::get_cycle_count(),
// //         }
// //     }
// // }
// 
// /// A measurement of a monotonically nondecreasing clock
// #[derive(Clone, Copy)]
// pub struct Instant {
//     now: u32,
// }
// 
// impl Instant {
//     /// Ticks elapsed since the `Instant` was created
//     pub fn elapsed(&self) -> u32 {
//         DWT::get_cycle_count().wrapping_sub(self.now)
//     }
// }
