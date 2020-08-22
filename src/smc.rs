//! SMC contoller setup

use crate::time::{PicoSeconds};
use crate::ebi::{ExternalBusInterface};
use crate::clock_gen::Clocks;
use crate::time::{Hertz};

use crate::target_device::{SMC, PMC};
use crate::target_device::smc::smc_cs_number::smc_mode::DBW_A as SmcDeviceBusWidth;
use crate::target_device::smc::smc_cs_number::smc_mode::BAT_A as SmcDeviceByteAccess;
use crate::target_device::smc::smc_cs_number::smc_mode::EXNW_MODE_A as SmcDeviceNwait;
use crate::target_device::smc::smc_cs_number::smc_mode::PS_A as SmcDevicePageSize;

pub enum SmcDeviceSelect{
	SmcDevice0,
	SmcDevice1,
	SmcDevice2,
	SmcDevice3,
}

impl SmcDeviceSelect {
	/// get device number
	pub fn nr(&self) -> usize {
		match self {
			SmcDeviceSelect::SmcDevice0 => 0,
			SmcDeviceSelect::SmcDevice1 => 1,
			SmcDeviceSelect::SmcDevice2 => 2,
			SmcDeviceSelect::SmcDevice3 => 3
		}
	}
}

pub struct SmcDeviceSetupTimings{
	pub read_cs : PicoSeconds,
	pub read : PicoSeconds,
	pub write_cs : PicoSeconds,
	pub write : PicoSeconds
}

pub struct SmcDevicePulseTimings{
	pub read_cs : PicoSeconds,
	pub read : PicoSeconds,
	pub write_cs : PicoSeconds,
	pub write : PicoSeconds
}

pub struct SmcDeviceCycleTimings{
	pub read : PicoSeconds,
	pub write : PicoSeconds
}

pub enum SmcDeviceWriteMode {
	WriteSignalNcs,
	WriteSignalNwe,
}

pub enum SmcDeviceReadMode {
	ReadSignalNcs,
	ReadSignalNrd,
}

pub struct SmcDeviceMode{
	pub dbw : SmcDeviceBusWidth,
	pub bat : SmcDeviceByteAccess,
	pub nwait : SmcDeviceNwait,
	pub ps : SmcDevicePageSize,
	pub read_mode : SmcDeviceReadMode,
	pub write_mode : SmcDeviceWriteMode,
}

// Todo: Add support for data scrambling per device, not a priority for v1.0 of this library

pub struct SmcDeviceConfig {
	pub mode  : SmcDeviceMode,
	pub setup : SmcDeviceSetupTimings,
	pub pulse : SmcDevicePulseTimings,
	pub cycle : SmcDeviceCycleTimings
}

#[derive(Debug)]
pub struct InvalidConfig;

fn calc_setup_val(clk: Hertz, delay: PicoSeconds) -> u8 {
	let mut v : u8 = 0;
	let cycles_duration:PicoSeconds = clk.into();
	let mut setup = cycles_duration.cycles(delay);

	if setup >= 128 {
		setup -= 128;
		v |= 1 << 5;
	}
	v |= setup as u8 & 0x1f;

	v
}

pub struct Smc {
	smc : SMC,
	clk : Hertz
}

/// SMC trait

impl Smc {
	/// Setup Static Memmory Controller
	pub fn setup(
		smc : SMC,
		_ebi : &ExternalBusInterface,
		clocks : &Clocks,
		pmc : &mut PMC
	) -> Self {
		// Enable SMC clock
		pmc.pmc_pcer0.write( |w| w.pid9().set_bit() );

		Smc {
			smc,
			clk : clocks.mck()
		}
	}

	/// Setup SMC Device with provided configuration
	pub fn set_device(
		&mut self,
		device : SmcDeviceSelect,
		config : SmcDeviceConfig
	){
		// write setup register
		unsafe {
			self.smc.smc_cs_number[device.nr()].smc_setup.write( |w| {
				w.ncs_rd_setup().bits(calc_setup_val(self.clk, config.setup.read_cs));
				w.nrd_setup().bits(calc_setup_val(self.clk, config.setup.read));
				w.ncs_wr_setup().bits(calc_setup_val(self.clk, config.setup.write_cs));
				w.nwe_setup().bits(calc_setup_val(self.clk, config.setup.write))
			});
		}

		// write pulse register

		// write cycle register

		// write mode register

	}
}
