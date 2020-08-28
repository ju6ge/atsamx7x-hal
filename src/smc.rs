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
	pub read_mode : SmcDeviceReadMode,
	pub write_mode : SmcDeviceWriteMode,
	pub ps : Option<SmcDevicePageSize>,
	// Todo: Add support for Data Float Time Optimisation
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


// The setup register may only hold values up to including 31 and 128 + 31 = 159 due to how the values are
// encoded in it
fn calc_setup_val(clk: Hertz, delay: PicoSeconds) -> u8 {
	let mut v : u8 = 0;
	let cycles_duration:PicoSeconds = clk.into();
	let mut setup = cycles_duration.cycles(delay);

	if setup >= 128 {
		setup -= 128;
		v |= 1 << 5;
	}
	assert!(setup <= 31);

	v |= setup as u8 & 0x1f;

	v
}

// The pulse register may only hold values up to including 63 and 256+63 = 319 due to how the values are
// encoded in it
fn calc_pulse_val(clk: Hertz, delay: PicoSeconds) -> u8 {
	let mut v : u8 = 0;
	let cycles_duration:PicoSeconds = clk.into();
	let mut pulse = cycles_duration.cycles(delay);

	if pulse >= 256 {
		pulse -= 256;
		v |= 1 << 6;
	}
	assert!(pulse <= 63);

	v |= pulse as u8 & 0x3f;

	v
}

// The cycle register may only hold values up to including 127 or 256+127 or 512+127 or 768+127
fn calc_cycle_val(clk: Hertz, delay: PicoSeconds) -> u16 {
	let mut v: u16 = 0;
	let cycles_duration:PicoSeconds = clk.into();
	let mut cycle = cycles_duration.cycles(delay);

	for i in (1..=3).rev() {
		if 256*i >= cycle {
			cycle -= 256*i;
			v |= (i as u16) << 7;
			break;
		}
	}

	assert!(cycle <= 127);

	v |= cycle as u16 & 0x7f;

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
	pub fn setup_device(
		&mut self,
		device : SmcDeviceSelect,
		config : SmcDeviceConfig
	) -> Result<(), InvalidConfig>{
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
		unsafe {
			self.smc.smc_cs_number[device.nr()].smc_pulse.write( |w| {
				w.ncs_rd_pulse().bits(calc_pulse_val(self.clk, config.pulse.read_cs));
				w.nrd_pulse().bits(calc_pulse_val(self.clk, config.pulse.read));
				w.ncs_wr_pulse().bits(calc_pulse_val(self.clk, config.pulse.write_cs));
				w.nwe_pulse().bits(calc_pulse_val(self.clk, config.pulse.write))
			});
		}

		// write cycle register
		unsafe {
			self.smc.smc_cs_number[device.nr()].smc_cycle.write( |w| {
				w.nrd_cycle().bits(calc_cycle_val(self.clk, config.cycle.read));
				w.nwe_cycle().bits(calc_cycle_val(self.clk, config.cycle.write))
			});
		}

		// write mode register
		self.smc.smc_cs_number[device.nr()].smc_mode.write( |w| {
			// bus width parameters
			match config.mode.dbw {
				SmcDeviceBusWidth::_8_BIT => {
					w.dbw()._8_bit();
				}
				SmcDeviceBusWidth::_16_BIT => {
					w.dbw()._16_bit();

					match config.mode.bat {
						SmcDeviceByteAccess::BYTE_SELECT => w.bat().byte_select(),
						SmcDeviceByteAccess::BYTE_WRITE => w.bat().byte_write()
					};
				}
			};

			match config.mode.nwait {
				SmcDeviceNwait::DISABLED => w.exnw_mode().disabled(),
				SmcDeviceNwait::FROZEN => w.exnw_mode().frozen(),
				SmcDeviceNwait::READY => w.exnw_mode().ready()
			};

			w
		});

		Ok(())
	}
}
