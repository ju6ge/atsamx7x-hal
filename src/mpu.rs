//! Memory Protection Unit

use cortex_m::{asm, peripheral::MPU};

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

impl MpuAccessPolicy {
	fn from_bits(bits : u8) -> Self{
		if bits == 0b01 || bits == 0b00 {
			Self::NoAccess
		} else if  bits == 0b11 {
			Self::ReadWrite
		} else {
			Self::ReadOnly
		}
	}
}

/// The caching policy for a "normal" memory region.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MpuCachePolicy{
	/// Non-cacheable memory region.
	NonCacheable,

	/// Write-through, no write allocate.
	WriteThrough,

	/// Write-back cacheable region.
	WriteBack {
		/// Whether a write miss loads the missed cache row into cache.
		write_allocate: bool,
	},

	// FIXME: There's also mixed "outer"/"inner" policies, but I don't know what that even means.
}

#[derive(Debug)]
pub struct InvalidReservedAttribute;

/// Describes memory type, cache policy, and shareability.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MpuMemoryAttributes {
	/// Shareable, non-cached, strongly-ordered memory region.
	StronglyOrdered,

	/// Non-cached device peripheral region.
	Device {
		/// Whether the region is accessible by more than one bus master (eg. a
		/// DMA engine or a second MCU core).
		shareable: bool,
	},

	/// Normal memory region (ie. "actual" memory, such as Flash or SRAM).
	Normal {
		/// Whether the region is accessible by more than one bus master
		/// (eg. a DMA engine or a second MCU core).
		shareable: bool,

		/// Cache policy of the region.
		cache_policy: MpuCachePolicy,
	}
}

impl MpuMemoryAttributes {
	//Turns `self` into its bit-level representation, consisting of the TEX, C, B, and S bits.
	fn to_bits(self) -> u32 {
		macro_rules! bits {
			( TEX=$tex:literal, C=$c:literal, B=$b:literal, S=$s:ident ) => {
				($tex << 3) | (if $s { 1 } else { 0 } << 2) | ($c << 1) | $b
			};
			( TEX=$tex:literal, C=$c:literal, B=$b:literal, S=$s:literal ) => {
				($tex << 3) | ($s << 2) | ($c << 1) | $b
			};
		}

		match self {
			Self::StronglyOrdered => bits!(TEX = 0b000, C = 0, B = 0, S = 0),
			Self::Device { shareable: false } => bits!(TEX = 0b010, C = 0, B = 0, S = 0),
			Self::Device { shareable: true } => bits!(TEX = 0b000, C = 0, B = 1, S = 0),
			Self::Normal { shareable, cache_policy, } =>
				match cache_policy {
					MpuCachePolicy::NonCacheable => bits!(TEX = 0b001, C = 0, B = 0, S = shareable),
					MpuCachePolicy::WriteThrough => bits!(TEX = 0b000, C = 1, B = 0, S = shareable),
					MpuCachePolicy::WriteBack { write_allocate: false, } => bits!(TEX = 0b000, C = 1, B = 1, S = shareable),
					MpuCachePolicy::WriteBack { write_allocate: true, } => bits!(TEX = 0b001, C = 1, B = 1, S = shareable),
				}
		}
	}

	//Read Attributes from bits
	fn from_bits(bits : u8) -> Result<Self, InvalidReservedAttribute> {
		let tex = (bits & 0x38) >> 3;
		let s = (bits & 0x04) >> 2;
		let c = (bits & 0x02) >> 1;
		let b = bits & 0x01;

		if tex == 0b000 && c == 0b0 && b == 0b0 {
			Ok(Self::StronglyOrdered)
		} else if tex == 0b000 && c == 0b0 && b == 0b1 {
			Ok(Self::Device {
				shareable : true
			})
		} else if tex == 0b010 && c == 0b0 && b == 0b0 {
			Ok(Self::Device {
				shareable : false
			})
		} else if tex == 0b000 && c == 0b1 && b == 0b0 {
			Ok(Self::Normal {
				shareable : if s == 1 { true } else { false },
				cache_policy : MpuCachePolicy::WriteThrough
			})
		} else if tex == 0b000 && c == 0b1 && b == 0b1 {
			Ok(Self::Normal {
				shareable : if s == 1 { true } else { false },
				cache_policy : MpuCachePolicy::WriteBack {
					write_allocate : false
				}
			})
		} else if tex == 0b001 && c == 0b1 && b == 0b1 {
			Ok(Self::Normal {
				shareable : if s == 1 { true } else { false },
				cache_policy : MpuCachePolicy::WriteBack {
					write_allocate : true
				}
			})
		} else if tex == 0b001 && c == 0b0 && b == 0b0 {
			Ok(Self::Normal {
				shareable : if s == 1 { true } else { false },
				cache_policy : MpuCachePolicy::NonCacheable
			})
		} else {
			Err(InvalidReservedAttribute)
		}
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
	pub const REGION32_B: Self = MpuRegionSize(4);
	pub const REGION64_B: Self = MpuRegionSize(5);
	pub const REGION128_B: Self = MpuRegionSize(6);
	pub const REGION256_B: Self = MpuRegionSize(7);
	pub const REGION512_B: Self = MpuRegionSize(8);
	pub const REGION1_K: Self = MpuRegionSize(9);
	pub const REGION2_K: Self = MpuRegionSize(10);
	pub const REGION4_K: Self = MpuRegionSize(11);
	pub const REGION8_K: Self = MpuRegionSize(12);
	pub const REGION16_K: Self = MpuRegionSize(13);
	pub const REGION32_K: Self = MpuRegionSize(14);
	pub const REGION64_K: Self = MpuRegionSize(15);
	pub const REGION128_K: Self = MpuRegionSize(16);
	pub const REGION256_K: Self = MpuRegionSize(17);
	pub const REGION512_K: Self = MpuRegionSize(18);
	pub const REGION1_M: Self = MpuRegionSize(19);
	pub const REGION2_M: Self = MpuRegionSize(20);
	pub const REGION4_M: Self = MpuRegionSize(21);
	pub const REGION8_M: Self = MpuRegionSize(22);
	pub const REGION16_M: Self = MpuRegionSize(23);
	pub const REGION32_M: Self = MpuRegionSize(24);
	pub const REGION64M_: Self = MpuRegionSize(25);
	pub const REGION128_M: Self = MpuRegionSize(26);
	pub const REGION256_M: Self = MpuRegionSize(27);
	pub const REGION512_M: Self = MpuRegionSize(28);
	pub const REGION1_G: Self = MpuRegionSize(29);
	pub const REGION2_G: Self = MpuRegionSize(30);
	pub const REGION4_G: Self = MpuRegionSize(31);

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

/// Subregion Disable (SRD) bits for the 8 subregions in a region.
///
/// Note that some cores do not support subregions for small region sizes. Check the core's User
/// Guide for more information.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MpuSubregions(u8);

impl MpuSubregions {
	/// None of the 8 subregions are enabled. Equivalent to disabling the entire region.
	pub const NONE: Self = MpuSubregions(0xff);

	/// All 8 subregions are enabled.
	pub const ALL: Self = MpuSubregions(0);

	/// Creates a `MpuSubregions` mask from raw Subregion Disable (SRD) bits.
	///
	/// The least significant bit disables the lowest 1/8th of the region, and so on.
	pub fn from_disable_bits(bits: u8) -> Self {
		MpuSubregions(bits)
	}

	/// Returns the raw 8-bit Subregion Disable Bits value.
	pub fn bits(self) -> u8 {
		self.0
	}
}

/// By default, all subregions are enabled.
impl Default for MpuSubregions {
	fn default() -> Self {
		Self::ALL
	}
}


/// Memory region description.
#[derive(Debug, Copy, Clone)]
pub struct ProtectedMemoryRegion {
	pub base_address : *const u32,
	pub size : MpuRegionSize,
	pub subregions : MpuSubregions,

	pub executable : bool,
	pub permissions : MpuAccessPolicy,
	pub attributes : MpuMemoryAttributes,
}

#[derive(Debug)]
pub struct MemoryRegionDisabled;

#[derive(Debug)]
pub struct MemoryRegionsFull;


/// MPU trait
pub trait Mpu {
	/// Enabele Memory Protection Unit
	fn enable(&mut self);

	/// Disable Memory Protection Unit
	fn disable(&mut self);

	fn is_enabled(&self) -> bool;

	/// Supported Region Number
	fn supported_regions(&self) -> u8;

	/// Count active regions
	fn count_regions(&mut self) -> u8;

	/// Read settings of region by number
	fn get_region(&mut self, rnr : u8) -> Result<ProtectedMemoryRegion, MemoryRegionDisabled>;

	/// Set Memory Region Options for specific region
	fn set_region(&mut self, region : ProtectedMemoryRegion, rnr : u8);

	/// Add Region to free MPU slot
	fn add_region(&mut self, region : ProtectedMemoryRegion) -> Result<(), MemoryRegionsFull>;
}

impl Mpu for MPU {
	fn get_region(&mut self, rnr : u8) -> Result<ProtectedMemoryRegion, MemoryRegionDisabled> {
		unsafe { self.rnr.write(rnr.into()) };

		let region_attributes = self.rasr.read();

		//check if memory region protection is enabled
		if (region_attributes & 0x1) == 0 {
			Err(MemoryRegionDisabled)

		//found enabled memory region
		} else {
			let base_address = self.rbar.read();

			let region = ProtectedMemoryRegion {
				// strip lower bits of address register, because they are irrelevant
				base_address : (base_address & 0xFFFF_FFF0) as *const u32,

				// size is encoded in bits 5 to 1
				size : MpuRegionSize((region_attributes & 0x0000_003D) as u8 >> 1),

				//subregions enable bits in bit 15 to 8
				subregions : MpuSubregions::from_disable_bits( ((region_attributes & 0x0000_FF00) >> 8) as u8 ),

				//permissions settings in bits 26 to 24
				permissions : MpuAccessPolicy::from_bits( ((region_attributes & 0x0700_0000) >> 24) as u8 ),

				// attributes in bits 21 to 16
				attributes : MpuMemoryAttributes::from_bits( ((region_attributes & 0x003F) >> 16) as u8 ).unwrap(),

				// executable bit is at position 28
				executable : (region_attributes & 0x1000_0000) == 0x1000_0000,
			};

			Ok(region)
		}
	}

	fn supported_regions(&self) -> u8 {
		let mpu_type = self._type.read();

		//support region count is encoded in bits 15 to 8
		((mpu_type & 0x0000_FF00) >> 8) as u8
	}

	fn count_regions(&mut self) -> u8 {
		let mut count = 0;

		for rnr in 0 .. self.supported_regions() {
			unsafe { self.rnr.write(rnr.into()) };

			let region_attributes = self.rasr.read();
			//check if memory region protection is enabled
			if (region_attributes & 0x1) == 1 {
				count += 1;
			}
		}

		count
	}

	fn enable(&mut self) {
		// https://developer.arm.com/docs/dui0553/latest/cortex-m4-peripherals/optional-memory-protection-unit/updating-an-mpu-region
		asm::dsb();
		
		//  last three bit in cntr register enable
		//  PRIVDEFENA | HFNMIENA | ENABLE

		//  PRIVDEFENA : privliedged code uses default memory map
		//  HFNMIENA   : enable MPU interrupt and exception handlers f
		//  ENABLE     : enable disable MPU
		//
		//  for now we will just enable all … todo figure out nice way of makeing this
		//  configurable
		unsafe {
			self.ctrl.modify( |r| {
				(r & 0xFFFF_FFF8) | 0x3
			});
		}

		// https://developer.arm.com/docs/dui0553/latest/cortex-m4-peripherals/optional-memory-protection-unit/updating-an-mpu-region
		asm::dsb();
		asm::isb();
	}

	fn disable(&mut self) {
		asm::dsb();

		unsafe {
			self.ctrl.write(0);
		}

		asm::dsb();
		asm::isb();
	}

	fn is_enabled(&self) -> bool {
		let ctrl = self.ctrl.read();
		if ctrl & 0x1 != 0 {
			true
		} else {
			false
		}
	}

	fn set_region(&mut self, region : ProtectedMemoryRegion, rnr : u8) {
		let mut mpu_enabled : bool = false;
		let ctrl = self.ctrl.read();

		//check if MPU is active
		if ctrl & 0x1 != 0 {
			mpu_enabled = true;

			// we need to disable the mpu while updating the region
			asm::dsb();
			unsafe {
				self.ctrl.write(0);
			}
		}

		//mask of last 4 bit, because they are not part of the address in this register
		let rbar : u32 = region.base_address as u32 & 0xFFFF_FFF0;

		let xn = if region.executable { 0 } else { 1 << 28 };
		let ap = (region.permissions as u32) << 24;
		let scb = region.attributes.to_bits() << 16;
		let srd = u32::from(region.subregions.bits()) << 8;
		let size = u32::from(region.size.bits()) << 1;
		let region_enable = 0x1;

		let rasr : u32 = xn | ap | scb | srd | size | region_enable;

		unsafe {
			//set region number
			self.rnr.write(rnr.into());

			//set base address
			self.rbar.write(rbar);

			//set region settings
			self.rasr.write(rasr);
		};

		if mpu_enabled {
			unsafe {
				self.ctrl.write(ctrl);
			}
			asm::dsb();
			asm::isb();
		}
	}

	fn add_region(&mut self, region : ProtectedMemoryRegion) -> Result<(), MemoryRegionsFull> {
		for rnr in 0 .. self.supported_regions() {
			unsafe { self.rnr.write(rnr.into()) };

			let region_attributes = self.rasr.read();
			//check if memory region is not used already
			if (region_attributes & 0x1) == 0 {
				self.set_region(region, rnr)
			}
		}

		Err(MemoryRegionsFull)
	}
}
