use crate::gpio::pioa::{PA18, PA20, PA0, PA15, PA16};
use crate::gpio::pioc::{PC0, PC1, PC2, PC3, PC4, PC5, PC6, PC7, PC18, PC15, PC20, PC21, PC22, PC23, PC24, PC25, PC26, PC27, PC28, PC29, PC31};
use crate::gpio::piod::{PD13, PD14, PD15, PD16, PD17, PD23, PD29};
use crate::gpio::pioe::{PE0, PE1, PE2, PE3, PE4, PE5};
use crate::gpio::{PeripheralCntr, PeriphA, PeriphC};
use crate::time::NanoSeconds;
use crate::clock_gen::Clocks;
use crate::delay::Delay;

use crate::target_device::{SDRAMC, PMC};
use embedded_hal::blocking::delay::{DelayUs};

use target_device::sdramc::sdramc_mr::MODE_A as SdramMode;


pub enum SdramColumns {
	Columns256,
	Columns512,
	Columns1K,
	Columns2K,
}

pub enum SdramRows {
	Rows2K,
	Rows4K,
	Rows8K
}

pub enum SdramBanks {
	Bank2,
	Bank4
}

pub enum SdramAlignment {
	Aligned,
	Unaligned
}

pub struct SdramTiming {
	twr : NanoSeconds,
	trc : NanoSeconds,
	trp : NanoSeconds,
	trcd : NanoSeconds,
	tras : NanoSeconds,
	txsr : NanoSeconds,
	refresh : NanoSeconds
}

pub struct SdramConfig {
	banks : SdramBanks,
	rows : SdramRows,
	columns : SdramColumns,
	alignment : SdramAlignment,
	timing : SdramTiming
}

#[derive(Debug)]
pub struct InvalidConfig;

pub trait SdramPins {}

pub trait Addr0 {}
pub trait Addr1 {}
pub trait Addr2 {}
pub trait Addr3 {}
pub trait Addr4 {}
pub trait Addr5 {}
pub trait Addr6 {}
pub trait Addr7 {}
pub trait Addr8 {}
pub trait Addr9 {}
pub trait Addr10 {}
pub trait Addr11 {}
pub trait Addr12 {}
pub trait Data0 {}
pub trait Data1 {}
pub trait Data2 {}
pub trait Data3 {}
pub trait Data4 {}
pub trait Data5 {}
pub trait Data6 {}
pub trait Data7 {}
pub trait Data8 {}
pub trait Data9 {}
pub trait Data10 {}
pub trait Data11 {}
pub trait Data12 {}
pub trait Data13 {}
pub trait Data14 {}
pub trait Data15 {}
pub trait Bank0{}
pub trait Bank1{}
pub trait NBS0 {}
pub trait NBS1 {}
pub trait CK {}
pub trait CKE {}
pub trait CS {}
pub trait RAS {}
pub trait CAS {}
pub trait WE {}


macro_rules! sdram_pins {
	( $($pin:ident: [$($inst:ty), *])+ ) => {
		$(
			$(
				impl $pin for $inst {}
			)*
		)+
	}
}

sdram_pins! {
	Addr0  : [ PC20<PeripheralCntr<PeriphA>> ]
	Addr1  : [ PC21<PeripheralCntr<PeriphA>> ]
	Addr2  : [ PC22<PeripheralCntr<PeriphA>> ]
	Addr3  : [ PC23<PeripheralCntr<PeriphA>> ]
	Addr4  : [ PC24<PeripheralCntr<PeriphA>> ]
	Addr5  : [ PC25<PeripheralCntr<PeriphA>> ]
	Addr6  : [ PC26<PeripheralCntr<PeriphA>> ]
	Addr7  : [ PC27<PeripheralCntr<PeriphA>> ]
	Addr8  : [ PC28<PeripheralCntr<PeriphA>> ]
	Addr9  : [ PC29<PeripheralCntr<PeriphA>> ]
	Addr10 : [ PD13<PeripheralCntr<PeriphC>> ]
	Addr11 : [ PC31<PeripheralCntr<PeriphA>> ]
	Addr12 : [ PA18<PeripheralCntr<PeriphC>> ]

	Data0  : [ PC0<PeripheralCntr<PeriphA>> ]
	Data1  : [ PC1<PeripheralCntr<PeriphA>> ]
	Data2  : [ PC2<PeripheralCntr<PeriphA>> ]
	Data3  : [ PC3<PeripheralCntr<PeriphA>> ]
	Data4  : [ PC4<PeripheralCntr<PeriphA>> ]
	Data5  : [ PC5<PeripheralCntr<PeriphA>> ]
	Data6  : [ PC6<PeripheralCntr<PeriphA>> ]
	Data7  : [ PC7<PeripheralCntr<PeriphA>> ]
	Data8  : [ PE0<PeripheralCntr<PeriphA>> ]
	Data9  : [ PE1<PeripheralCntr<PeriphA>> ]
	Data10 : [ PE2<PeripheralCntr<PeriphA>> ]
	Data11 : [ PE3<PeripheralCntr<PeriphA>> ]
	Data12 : [ PE4<PeripheralCntr<PeriphA>> ]
	Data13 : [ PE5<PeripheralCntr<PeriphA>> ]
	Data14 : [ PA15<PeripheralCntr<PeriphA>> ]
	Data15 : [ PA16<PeripheralCntr<PeriphA>> ]

	Bank0  : [ PA20<PeripheralCntr<PeriphC>> ]
	Bank1  : [ PA0<PeripheralCntr<PeriphC>> ]

	NBS0   : [ PC18<PeripheralCntr<PeriphA>> ]
	NBS1   : [ PD15<PeripheralCntr<PeriphC>> ]

	CK     : [ PD23<PeripheralCntr<PeriphC>> ]
	CKE    : [ PD14<PeripheralCntr<PeriphC>> ]
	CS     : [ PC15<PeripheralCntr<PeriphA>> ]
	RAS    : [ PD16<PeripheralCntr<PeriphC>> ]
	CAS    : [ PD17<PeripheralCntr<PeriphC>> ]
	WE     : [ PD29<PeripheralCntr<PeriphC>> ]
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12,
	D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12,
	D13, D14, D15, B0, B1, N0, N1, CKT, CKET, CST, RAST, CAST, WET> SdramPins

	for (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12,
		D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, D10, D11, D12,
		D13, D14, D15, B0, B1, N0, N1, CKT, CKET, CST, RAST, CAST, WET)

	 where
		A0 : Addr0,
		A1 : Addr1,
		A2 : Addr2,
		A3 : Addr3,
		A4 : Addr4,
		A5 : Addr5,
		A6 : Addr6,
		A7 : Addr7,
		A8 : Addr8,
		A9 : Addr9,
		A10 : Addr10,
		A11 : Addr11,
		A12 : Addr12,
		D0 : Data0,
		D1 : Data1,
		D2 : Data2,
		D3 : Data3,
		D4 : Data4,
		D5 : Data5,
		D6 : Data6,
		D7 : Data7,
		D8 : Data8,
		D9 : Data9,
		D10 : Data10,
		D11 : Data11,
		D12 : Data12,
		D13 : Data13,
		D14 : Data14,
		D15 : Data15,
		B0 : Bank0,
		B1 : Bank1,
		N0 : NBS0,
		N1 : NBS1,
		CKT : CK,
		CKET : CKE,
		CST : CS,
		RAST : RAS,
		CAST : CAS,
		WET : WE
	{}

pub struct Sdram<PINS> {
	sdramc : SDRAMC,
	pins: PINS,
	start_address : *const u32,
	size : u32,
	mode : SdramMode
}

impl<PINS> Sdram<PINS> {
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

		//Todo calculate correct timing parameters

		sdramc.sdramc_cr.write( |w| {
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
			w.dbw().set_bit()
			//Todo add Timing parameters
		});

		let mut delay = Delay::new(unsafe{cortex_m::Peripherals::steal()}.SYST, clocks);
		delay.delay_us(200 as u32);

		let mut sdram = Sdram{
		                    sdramc,
		                    pins,
		                    start_address : 0x7000_0000 as *const u32, //start address defined by the hardware
		                    size : 0x0200_0000, // 256 MB
		                    mode : SdramMode::NORMAL
						};

		sdram.setMode(SdramMode::NOP);
		//read mode + mem barrier + write to sdram

		sdram.setMode(SdramMode::ALLBANKS_PRECHARGE);
		//read mode + mem barrier + write to sdram

		sdram.setMode(SdramMode::AUTO_REFRESH);
		//read mode + mem barrier + write to sdram x8

		//Todo missing steps 7-11 from Datasheet

		//return sdram
		Ok(sdram)
	}

	pub fn setMode(&mut self, mode : SdramMode) {
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

	pub fn release(self) -> (SDRAMC, PINS) {
		(self.sdramc, self.pins)
	}
}