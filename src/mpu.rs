//! Memory Protection Unit

use crate::target_device::{MPU};

/// Data access permissions for a memory region from unprivileged code.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MpuAccessPolicy {
	/// Any data access (read or write) will generate a fault.
	NoAccess = 0b01,

	/// Any write access will generate a fault.
	ReadOnly = 0b10,

	/// Region unprotected, both reads and writes are allowed.
	ReadWrite = 0b11,
}

/// The caching policy for a "normal" memory region.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MpuCachePolicy{
	/// Write-through, no write allocate
	WriteThrough,

	/// Write-back cacheable region, no write allocate.
	WriteBack,

	/// Not cacheable region
	NoCache
}

/// Describes memory type, cache policy, and shareability.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MpuMemoryAttributes {
	/// Shareable, non-cached, strongly-ordered memory region.
	StronglyOrdered,

	/// Non-cached device peripheral region. Always considered shareable.
	Device,

	/// Normal memory region (ie. "actual" memory, such as Flash or SRAM).
	Normal {
		/// Whether the region is accessible by more than one bus master
		/// (eg. a DMA engine or a second MCU core).
		shareable: bool,

		/// Cache policy of the region.
		cache_policy: MpuCachePolicy,
	}
}

/// Memory region size value (5 bits).
///
/// Memory regions must have a size that is a power of two, and their base address must be naturally
/// aligned (ie. aligned to their size).
///
/// There is a core-specific minimum size exposed as `Mpu::MIN_REGION_SIZE`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MpuRegionSize(u8);

impl MpuRegionSize {
	pub const S32B: Self = MpuRegionSize(4);
	pub const S64B: Self = MpuRegionSize(5);
	pub const S128B: Self = MpuRegionSize(6);
	pub const S256B: Self = MpuRegionSize(7);
	pub const S512B: Self = MpuRegionSize(8);
	pub const S1K: Self = MpuRegionSize(9);
	pub const S2K: Self = MpuRegionSize(10);
	pub const S4K: Self = MpuRegionSize(11);
	pub const S8K: Self = MpuRegionSize(12);
	pub const S16K: Self = MpuRegionSize(13);
	pub const S32K: Self = MpuRegionSize(14);
	pub const S64K: Self = MpuRegionSize(15);
	pub const S128K: Self = MpuRegionSize(16);
	pub const S256K: Self = MpuRegionSize(17);
	pub const S512K: Self = MpuRegionSize(18);
	pub const S1M: Self = MpuRegionSize(19);
	pub const S2M: Self = MpuRegionSize(20);
	pub const S4M: Self = MpuRegionSize(21);
	pub const S8M: Self = MpuRegionSize(22);
	pub const S16M: Self = MpuRegionSize(23);
	pub const S32M: Self = MpuRegionSize(24);
	pub const S64M: Self = MpuRegionSize(25);
	pub const S128M: Self = MpuRegionSize(26);
	pub const S256M: Self = MpuRegionSize(27);
	pub const S512M: Self = MpuRegionSize(28);
	pub const S1G: Self = MpuRegionSize(29);
	pub const S2G: Self = MpuRegionSize(30);
	/// The entire 4 GiB memory space.
	pub const S4G: Self = MpuRegionSize(31);

	/// Creates a `MpuRegionSize` from a raw 5-bit value.
	///
	/// The `bits` encode a region size of `2^(bits + 1)`. For example, a 1 KiB region would use
	/// `0b01001` (9): `2^(9+1) = 2^10 = 1024`.
	pub const fn from_raw_bits(bits: u8) -> Self {
		MpuRegionSize(bits)
	}

	/// Returns the raw 5-bit value encoding the region size.
	pub const fn bits(self) -> u8 {
		self.0
	}
}

/// Memory region description.
#[derive(Debug, Copy, Clone)]
pub struct ProtectedMemoryRegion {
	pub base_address : *const u32,
	pub size : MpuRegionSize,

	pub executable : bool,
	pub permissions : MpuAccessPolicy,
	pub attributes : MpuMemoryAttributes,
}

#[derive(Debug)]
pub struct MemoryRegionDisabled;


/// MPU trait
pub trait Mpu {
	fn get_mpu_region(&mut self, rnr : u8) -> Result<ProtectedMemoryRegion, MemoryRegionDisabled>;
	//fn enable();
	//fn disable();
}

impl Mpu for MPU {
	fn get_mpu_region(&mut self, rnr : u8) -> Result<ProtectedMemoryRegion, MemoryRegionDisabled> {
		unsafe {self.rnr.write(rnr.into())};

		let region_attributes = self.rasr.read();

		//check if memory region protection is enabled
		if (region_attributes & 0x1) == 0 {
			Err(MemoryRegionDisabled)

		//found enabled memory region
		} else {
			let base_address = self.rbar.read();

			let region = ProtectedMemoryRegion {
				base_address : (base_address & 0xFFFF_FFF0) as *const u32,
			};
			Ok(region)
		}
	}
}
