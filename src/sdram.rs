//! SDRAM controller setup

// To see an example of how to use this abstraction take a look at:
//
// https://github.com/ju6ge/hd_embedded_rust

use crate::time::{PicoSeconds};
use crate::clock_gen::Clocks;
use crate::delay::Delay;

use crate::target_device::{SDRAMC, PMC};
use embedded_hal::blocking::delay::{DelayUs};

use target_device::sdramc::sdramc_mr::MODE_A as SdramMode;
use cortex_m::asm;

pub enum SdramColumns {
	Columns256,
	Columns512,
	Columns1K,
	Columns2K,
}

impl SdramColumns {
	pub fn addressing_bits(&self) -> u32 {
		match self {
			SdramColumns::Columns256 => 8,
			SdramColumns::Columns512 => 9,
			SdramColumns::Columns1K=> 10,
			SdramColumns::Columns2K => 11
		}
	}
}

pub enum SdramRows {
	Rows2K,
	Rows4K,
	Rows8K
}

impl SdramRows {
	pub fn addressing_bits(&self) -> u32 {
		match self {
			SdramRows::Rows2K => 11,
			SdramRows::Rows4K => 12,
			SdramRows::Rows8K => 13
		}
	}
}

pub enum SdramBanks {
	Bank2,
	Bank4
}

impl SdramBanks {
	pub fn addressing_bits(&self) -> u32 {
		match self {
			SdramBanks::Bank2 => 1,
			SdramBanks::Bank4 => 2
		}
	}
}

pub enum SdramCasLatency{
	Latency1,
	Latency2,
	Latency3
}

pub enum SdramAlignment {
	Aligned,
	Unaligned
}

pub struct SdramTiming {
	pub twr : PicoSeconds,
	pub trc : PicoSeconds,
	pub trp : PicoSeconds,
	pub trcd : PicoSeconds,
	pub tras : PicoSeconds,
	pub txsr : PicoSeconds,
	pub refresh : PicoSeconds
}

pub struct SdramConfig {
	pub banks : SdramBanks,
	pub rows : SdramRows,
	pub columns : SdramColumns,
	pub alignment : SdramAlignment,
	pub latency : SdramCasLatency,
	pub timing : SdramTiming
}

#[derive(Debug)]
pub struct InvalidConfig;

/// Trait to for implement for Pin Configuration
pub trait SdramPins {}

pub struct Sdram<PINS> {
	sdramc : SDRAMC,
	pins: PINS,
	start_address : *const u32,
	size : u32,
	mode : SdramMode
}

impl<PINS> Sdram<PINS> {
	/// Perform Software Initialisation of the SDRAM
	pub fn setup(
		sdramc : SDRAMC,
		pins : PINS,
		config : SdramConfig,
		clocks : &Clocks,
		pmc : &mut PMC
	) -> Result<Self, InvalidConfig>
	where
		PINS: SdramPins
	{
		//enable sdram address area
		let matrix = unsafe { &(*target_device::MATRIX::ptr()) };
		matrix.ccfg_smcnfcs.modify( |_,w| w.sdramen().set_bit() );

		//Enable SDRAM Clock
		pmc.pmc_pcer1.write( |w| w.pid62().set_bit() );

		let cycle_duration : PicoSeconds = clocks.mck().into();

		sdramc.sdramc_cr.write( |w| {
			//configure size specification
			match config.columns {
				SdramColumns::Columns256 => w.nc().col8(),
				SdramColumns::Columns512 => w.nc().col9(),
				SdramColumns::Columns1K  => w.nc().col10(),
				SdramColumns::Columns2K  => w.nc().col11(),
			};
			match config.rows {
				SdramRows::Rows2K => w.nr().row11(),
				SdramRows::Rows4K => w.nr().row12(),
				SdramRows::Rows8K => w.nr().row13(),
			};
			match config.banks {
				SdramBanks::Bank2 => w.nb().bank2(),
				SdramBanks::Bank4 => w.nb().bank4(),
			};

			// make sure to be in 16 bit mode since this architechture only supports 16bit wide data access
			w.dbw().set_bit();

			// Timing conifiguration
			match config.latency {
				SdramCasLatency::Latency1 => w.cas().latency1(),
				SdramCasLatency::Latency2 => w.cas().latency2(),
				SdramCasLatency::Latency3 => w.cas().latency3(),
			};
			unsafe {
				w.twr().bits(cycle_duration.cycles(config.timing.twr) as u8);
				w.trc_trfc().bits(cycle_duration.cycles(config.timing.trc) as u8);
				w.trp().bits(cycle_duration.cycles(config.timing.trp) as u8);
				w.trcd().bits(cycle_duration.cycles(config.timing.trcd) as u8);
				w.tras().bits(cycle_duration.cycles(config.timing.tras) as u8);
				w.txsr().bits(cycle_duration.cycles(config.timing.txsr) as u8)
			}
		});

		//alignment
		sdramc.sdramc_cfr1.modify(|_,w| {
			match config.alignment {
				SdramAlignment::Unaligned => w.unal().clear_bit(),
				SdramAlignment::Aligned   => w.unal().set_bit(),
			}
		});

		sdramc.sdramc_mdr.write(|w| w.md().sdram() );

		let mut delay = Delay::new(unsafe{cortex_m::Peripherals::steal()}.SYST, clocks);
		delay.delay_us(200 as u32);

		let addressing_bits = config.banks.addressing_bits() + config.rows.addressing_bits() + config.columns.addressing_bits();
		
		let mut sdram = Sdram{
		                    sdramc,
		                    pins,
		                    start_address : 0x7000_0000 as *const u32, //start address defined by the hardware
		                    size : 1 << addressing_bits,
		                    mode : SdramMode::NORMAL
						};

		let mem_addr = sdram.start_address as *mut u32;

		// Initialisation steps from Datasheet
		// each step:
		//     set mode
		//     read mode
		//     mem barrier
		//     write to sdram
		sdram.set_mode(SdramMode::NOP);
		let _ = sdram.sdramc.sdramc_mr.read().mode().bits();
		asm::dmb();
		unsafe { core::ptr::write_volatile(mem_addr, 0); }

		sdram.set_mode(SdramMode::ALLBANKS_PRECHARGE);
		let _ = sdram.sdramc.sdramc_mr.read().mode().bits();
		asm::dmb();
		unsafe { core::ptr::write_volatile(mem_addr, 1); }

		sdram.set_mode(SdramMode::AUTO_REFRESH);
		let _ = sdram.sdramc.sdramc_mr.read().mode().bits();
		asm::dmb();
		for i in 0..8 {
			unsafe { core::ptr::write_volatile(mem_addr, i); }
		}

		sdram.set_mode(SdramMode::LOAD_MODEREG);
		let _ = sdram.sdramc.sdramc_mr.read().mode().bits();
		asm::dmb();
		unsafe { core::ptr::write_volatile(mem_addr, 2); }

		//Missing Step for mobile sdram initialisation maybe add this in the future
		//-> but will likely require additions to the conifiguration

		sdram.set_mode(SdramMode::NORMAL);
		let _ = sdram.sdramc.sdramc_mr.read().mode().bits();
		asm::dmb();
		unsafe { core::ptr::write_volatile(mem_addr, 3); }

		//enabele refresh
		sdram.sdramc.sdramc_tr.write(|w| unsafe{ w.count().bits(cycle_duration.cycles(config.timing.refresh) as u16 ) });

		//return sdram
		Ok(sdram)
	}

	pub fn set_mode(&mut self, mode : SdramMode) {
		self.sdramc.sdramc_mr.write( |w| {
			match mode {
				SdramMode::NORMAL             => w.mode().normal(),
				SdramMode::NOP                => w.mode().nop(),
				SdramMode::ALLBANKS_PRECHARGE => w.mode().allbanks_precharge(),
				SdramMode::LOAD_MODEREG       => w.mode().load_modereg(),
				SdramMode::AUTO_REFRESH       => w.mode().auto_refresh(),
				SdramMode::EXT_LOAD_MODEREG   => w.mode().ext_load_modereg(),
				SdramMode::DEEP_POWERDOWN     => w.mode().deep_powerdown(),
			}
		});
		self.mode = mode;
	}

	pub fn start_address(self) -> *const u32 {
		self.start_address
	}

	pub fn size(self) -> u32 {
		self.size
	}

	pub fn release(self) -> (SDRAMC, PINS) {
		(self.sdramc, self.pins)
	}
}
