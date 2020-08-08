//! SMC contoller setup

use crate::time::{PicoSeconds};

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
	pub fn value(&self) -> u32 {
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
	pub mode : SmcDeviceMode,
	pub setup : SmcDeviceSetupTimings,
	pub pulse : SmcDevicePulseTimings,
	pub cycle : SmcDeviceCycleTimings
}

