//! Clock configuration

use crate::time::*;
use crate::target_device::PMC;
use crate::target_device::SUPC;
use crate::target_device::UTMI;

enum ClockCalcStrategy {
	FromFrequency,
	FromDivider
}

/// Enum to select SLCK source
///
/// SLK can be driven by two different clocks at 32MHz
/// one of the clocks can be bypassed using an external clock
/// this gives us three different modes of operation
pub enum SlckSrc{
	SlowRC,
	CrystalOscillator,
	Bypass
}

#[derive(Debug)]
pub struct InvalidConfig;

/// Holds the current SLCK config
pub struct SlckConfig {
	src : SlckSrc,
	freq : Hertz
}

impl SlckConfig {
	// use the slow rc oscillator
	pub fn use_rc(mut self) -> Self {
		self.src = SlckSrc::SlowRC;
		self.freq = KiloHertz(32).into();

		self
	}
	// use the more precise crystal oscillator
	pub fn use_crystal(mut self) -> Self {
		self.src = SlckSrc::CrystalOscillator;
		self.freq = Hertz(32768);

		self
	}

	/// bypass and use own clk src -> you need to provide the frequency of that clk src!
	pub fn bypass(mut self, freq:Hertz) -> Self {
		self.src = SlckSrc::Bypass;
		self.freq = freq;

		self
	}
}

impl Default for SlckConfig {
	fn default() -> SlckConfig {
		SlckConfig{
			src: SlckSrc::SlowRC,
			freq: KiloHertz(32).into()
		}
	}
}

/// Enum to select MAINCK source
///
/// There are 2 different sources for MAINCK one of them can
/// be bypassed which leaves 3 options to select from.
pub enum MainckSrc {
	MainRC,
	MainCrystalOscillator,
	Bypass
}

pub enum MainRcFreq {
	Freq4Mhz,
	Freq8Mhz,
	Freq12Mhz
}

/// Holds the current MAINCK config
pub struct MainckConfig {
	rc_on : bool,
	crystal_on : bool,
	rc_freq : MainRcFreq,
	freq : Hertz,
	src : MainckSrc,
	startup_cycles : u8
}

impl MainckConfig {
	/// use rc oscillator at 4MHz
	pub fn use_rc_4mhz(mut self) -> Self {
		self.src = MainckSrc::MainRC;
		self.rc_on = true;
		self.rc_freq = MainRcFreq::Freq4Mhz;

		self
	}

    /// use rc oscillator at 8MHz
	pub fn use_rc_8mhz(mut self) -> Self {
		self.src = MainckSrc::MainRC;
		self.rc_on = true;
		self.rc_freq = MainRcFreq::Freq8Mhz;

		self
	}

    /// use rc oscillator at 12MHz
	pub fn use_rc_12mhz(mut self) -> Self {
		self.src = MainckSrc::MainRC;
		self.rc_on = true;
		self.rc_freq = MainRcFreq::Freq12Mhz;

		self
	}

	/// disable rc oscillator
	pub fn disable_rc(mut self) -> Self {
		self.rc_on = false;

		self
	}

	/// The frequency of the crystal oscillator depends on how the devices is connected
	/// so you need to provide the frequency you configured with the hardware
	pub fn use_crystal(mut self, freq:Hertz) -> Self {
		self.src = MainckSrc::MainCrystalOscillator;
		self.freq = freq;
		self.crystal_on = true;

		self
	}

	/// disable crystal oscillator
	pub fn disable_crystal(mut self) -> Self {
		self.crystal_on = false;

		self
	}

	/// bypass and use own clk src -> you need to provide the frequency of that clk src!
	pub fn bypass(mut self, freq:Hertz) -> Self {
		self.src = MainckSrc::Bypass;
		self.freq = freq;
		self.crystal_on = false;

		self
	}


	/// procide slck cycle count to wait for crystal oscillator to stabilize
	pub fn startup_cycles(mut self, cycles:u8) -> Self {
		self.startup_cycles = cycles;

		self
	}
}

impl Default for MainckConfig {
	fn default() -> MainckConfig {
		MainckConfig {
			src : MainckSrc::MainRC,
			rc_freq : MainRcFreq::Freq12Mhz,
			rc_on : true,
			crystal_on : false,
			freq : MegaHertz(12).into(),
			startup_cycles : 100
		}
	}
}

/// Holds the current PLLA clock config
pub struct PllackConfig {
	freq : Hertz,
	diva : u8,
	mula : u16,
	strategy : ClockCalcStrategy,
	startup_cycles : u8
}

impl PllackConfig {
	/// configure PLLA based on target frequency, tries to calculate reasonable values for the divider
	pub fn from_freq(mut self, freq:Hertz) -> Self{
		self.freq = freq;
		self.diva = 0;
		self.mula = 0;
		self.strategy = ClockCalcStrategy::FromFrequency;

		self
	}

	/// configure PLLA based on divider values, will calculate the resulting frequency
	pub fn from_divider(mut self, diva:u8, mula:u16 ) -> Self {
		self.freq = Hertz(0);
		self.diva = diva;
		self.mula = mula;
		self.strategy = ClockCalcStrategy::FromDivider;

		self
	}

	pub fn startup_cycles(mut self, cycles:u8) -> Self {
		self.startup_cycles = cycles;

		self
	}
}

impl Default for PllackConfig {
	/// default for this is in disabled state since no sensible value can be assumed
	fn default() -> PllackConfig {
		PllackConfig {
			freq : Hertz(0),
			diva : 0,
			mula : 0,
			strategy : ClockCalcStrategy::FromDivider,
			startup_cycles : 0
		}
	}
}

pub enum UpllckSrcFreq {
	SRC12MHz,
	SRC16MHz
}

/// Holds the current UPLL clock config
pub struct UpllckConfig {
	src : UpllckSrcFreq,
	startup_cycles : u8,
	enable : bool
}

impl UpllckConfig {
	pub fn enable(mut self) -> Self {
		self.enable = true;

		self
	}

	pub fn disable(mut self) -> Self {
		self.enable = false;

		self
	}

	pub fn src_freq(mut self, src:UpllckSrcFreq) -> Self {
		self.src = src;

		self
	}

	pub fn startup_cycles(mut self, cycles:u8) -> Self {
		self.startup_cycles = cycles;

		self
	}
}

impl Default for UpllckConfig {
	fn default() -> UpllckConfig {
		UpllckConfig {
			src : UpllckSrcFreq::SRC12MHz,
			startup_cycles : 0,
			enable : false
		}
	}
}

pub enum MasterClockSrc {
	SLCK,
	MAINCK,
	PLLACK,
	UPLLCKDIV,
}

pub enum MasterPrescale {
	Pres1,
	Pres2,
	Pres3,
	Pres4,
	Pres8,
	Pres16,
	Pres32,
	Pres64
}

impl MasterPrescale {
	pub fn to_value(&self) -> u32 {
		match self {
			Self::Pres1 => 1,
			Self::Pres2 => 2,
			Self::Pres3 => 3,
			Self::Pres4 => 4,
			Self::Pres8 => 8,
			Self::Pres16 => 16,
			Self::Pres32 => 32,
			Self::Pres64 => 64
		}
	}
}

pub enum MasterDivider {
	Div1,
	Div2,
	Div3,
	Div4
}

impl MasterDivider {
	pub fn to_value(&self) -> u32 {
		match self {
			Self::Div1 => 1,
			Self::Div2 => 2,
			Self::Div3 => 3,
			Self::Div4 => 4
		}
	}
}

pub enum UpllDiv{
	Div1,
	Div2
}

/// Holds the configuration of the Master Clock driving the CPU and Peripherals
pub struct MasterClockConfig {
	src : MasterClockSrc,
	pres : MasterPrescale,
	mdiv : MasterDivider,
	uplldiv : UpllDiv,
}


impl MasterClockConfig {
	/// select slow clock as source
	pub fn src_slck(mut self) -> Self {
		self.src = MasterClockSrc::SLCK;

		self
	}

	/// select main clock as source
	pub fn src_mainck(mut self) -> Self {
		self.src = MasterClockSrc::MAINCK;

		self

	}

	/// select plla clock as source
	pub fn src_pllack(mut self) -> Self {
		self.src = MasterClockSrc::PLLACK;

		self

	}

	/// select uplldiv clock as source
	pub fn src_upllckdiv(mut self) -> Self {
		self.src = MasterClockSrc::UPLLCKDIV;

		self

	}

	/// set divider for uplldiv clock signal
	pub fn set_uplldiv(mut self, div:UpllDiv) -> Self {
		self.uplldiv = div;

		self
	}

	/// set divider values for processor and perpheral clocks
	pub fn from_divider(mut self, pres:MasterPrescale, mdiv:MasterDivider) -> Self {
		self.pres = pres;
		self.mdiv = mdiv;

		self
	}
}

impl Default for MasterClockConfig {
	fn default() -> MasterClockConfig {
		MasterClockConfig{
			src : MasterClockSrc::MAINCK,
			pres : MasterPrescale::Pres1,
			mdiv : MasterDivider::Div1,
			uplldiv : UpllDiv::Div1,
		}
	}
}

/// Holds the configuration of all main clock domains
pub struct SystemClockConfig {
	pub slck_conf : SlckConfig,
	pub mainck_conf : MainckConfig,
	pub plla_conf : PllackConfig,
	pub upll_conf : UpllckConfig,
	pub mck_conf : MasterClockConfig
}

impl SystemClockConfig {
	/// Freezes the clock configuration by making it effective
	pub fn freeze(&self, pmc: &mut PMC, supc: &mut SUPC) -> Clocks {
		// Slow Clock configuration
		match self.slck_conf.src {
			SlckSrc::SlowRC => {
				// Dont do anything here, since this will be run at startup the SlowRC is already selected
				// It is not possible to switch back to the SlowRC!
			}
			SlckSrc::CrystalOscillator => {
				supc.supc_cr.write( |w| {
					w.key().passwd();
					w.xtalsel().set_bit()
				});
			}
			SlckSrc::Bypass => {
				supc.supc_mr.write( |w| {
					w.key().passwd();
					w.oscbypass().set_bit()
				});
				supc.supc_cr.write( |w| {
					w.key().passwd();
					w.xtalsel().set_bit()
				});
			}
		}

		// Main Clock configuration

		// if main rc should be enabled set it up
		if self.mainck_conf.rc_on {
			if pmc.ckgr_mor.read().moscrcen().bit_is_clear() {
				pmc.ckgr_mor.modify( |_, w| {
					w.key().passwd();
					w.moscrcen().set_bit()
				});
			}
			while pmc.pmc_sr.read().moscrcs().bit_is_clear(){
				//Wait until clock has stabilized
			}
			pmc.ckgr_mor.modify( |_,w| {
				w.key().passwd();
				match self.mainck_conf.rc_freq {
					MainRcFreq::Freq4Mhz => w.moscrcf()._4_mhz(),
					MainRcFreq::Freq8Mhz => w.moscrcf()._8_mhz(),
					MainRcFreq::Freq12Mhz => w.moscrcf()._12_mhz()
				}
			});
			while pmc.pmc_sr.read().moscrcs().bit_is_clear(){
				//Wait until clock has stabilized
			}
		}

		// if main crystal oscillator is enabled set it up
		if self.mainck_conf.crystal_on {
			if pmc.ckgr_mor.read().moscxten().bit_is_clear() {
				pmc.ckgr_mor.modify( |_, w| {
					w.key().passwd();
					unsafe{ w.moscxtst().bits(self.mainck_conf.startup_cycles); }
					w.moscxten().set_bit()
				});
			}
			while pmc.pmc_sr.read().moscxts().bit_is_clear(){
				//Wait until clock has stabilized
			}
		}

		// select clock
		let mainck_freq:Hertz;

		match self.mainck_conf.src {
			MainckSrc::MainRC => {
				if !self.mainck_conf.rc_on {
					panic!("Can not select Main RC Oscillator as MAINCK src if it is disabled!");
				}
				pmc.ckgr_mor.modify( |_,w| {
					w.key().passwd();
					w.moscsel().clear_bit()
				});
				while pmc.pmc_sr.read().moscsels().bit_is_clear(){
					//Wait for switch to be complete
				}
				mainck_freq = match self.mainck_conf.rc_freq {
					MainRcFreq::Freq4Mhz => MegaHertz(4).into(),
					MainRcFreq::Freq8Mhz => MegaHertz(8).into(),
					MainRcFreq::Freq12Mhz => MegaHertz(12).into()
				};
			}
			MainckSrc::MainCrystalOscillator => {
				if !self.mainck_conf.crystal_on {
					panic!("Can not select Crystal Oscillator as MAINCK src if it is disabled!");
				}
				pmc.ckgr_mor.modify( |_,w| {
					w.key().passwd();
					w.moscsel().set_bit()
				});
				while pmc.pmc_sr.read().moscsels().bit_is_clear(){
					//Wait for switch to be complete
				}
				mainck_freq = self.mainck_conf.freq;
			}
			MainckSrc::Bypass => {
				if self.mainck_conf.crystal_on {
					panic!("Can not bypass crystal oscillator when it is enabled!");
				}

				// if you use the bypass make sure your oscillator has stabilized before usage!

				pmc.ckgr_mor.modify( |_,w| {
					w.key().passwd();
					w.moscxtby().set_bit();
					w.moscxten().clear_bit()
				});
				pmc.ckgr_mor.modify( |_,w| {
					w.key().passwd();
					w.moscsel().set_bit()
				});
				while pmc.pmc_sr.read().moscsels().bit_is_clear(){
					//Wait for switch to be complete
				}
				mainck_freq = self.mainck_conf.freq;
			}
		}
		// shut down disabled oscillators
		if !self.mainck_conf.rc_on {
			pmc.ckgr_mor.modify( |_,w| {
				w.key().passwd();
				w.moscrcen().clear_bit()
			});
		}
		if !self.mainck_conf.crystal_on {
			pmc.ckgr_mor.modify( |_,w| {
				w.key().passwd();
				w.moscxten().clear_bit()
			});
		}

		// Plla configuration
		let plla_freq:Hertz;
		match self.plla_conf.strategy {
			ClockCalcStrategy::FromDivider => {
				pmc.ckgr_pllar.write( |w| {
					w.one().set_bit();
					unsafe { w.mula().bits(self.plla_conf.mula) };
					unsafe { w.diva().bits(self.plla_conf.diva) };
					unsafe { w.pllacount().bits(self.plla_conf.startup_cycles) }
				});
				while pmc.pmc_sr.read().locka().bit_is_clear() {
					//Wait until clock has stabilized
				}

				plla_freq = Hertz(mainck_freq.0 * (self.plla_conf.mula + 1) as u32 / (self.plla_conf.diva) as u32);
			}
			ClockCalcStrategy::FromFrequency => {
		    	let factor : f32 =  self.plla_conf.freq.0 as f32 / mainck_freq.0 as f32;

		    	let mula:u16;
		    	let diva:u8;

		    	//special case for case out_freq = in_freq since plla muliplies with at least two
		    	if factor == 1.0 {
			    	mula = 1;
					diva = 2;
		    	//if factor is an integer use it als multiplier with divisor 1
		    	} else if factor == (factor as u32) as f32 {
					mula = factor as u16 - 1;
					diva = 1;
		    	} else {
					let mut mul : Option<u16> = None;
					let mut div : Option<u16> = None;
					let mut err : Option<f32> = None;
					let mut d:u16 = 1;
					let mut m:u16 = 0;
					while d<=255 && m<63 {
						m = (d as f32 * factor) as u16;
						let e:f32 = m as f32 / d as f32 - factor;
						if m > 63 || m == 1 {
							d += 1;
							continue;
						}
						if e == 0.0 {
							mul = Some(m);
							div = Some(d);
							break;
						}
						if err.is_none() || err.unwrap() > e {
							mul = Some(m);
							div = Some(d);
							err = Some(e);
						}
						d += 1;
					}

					if mul.is_none() || div.is_none() {
						panic!("No suitable multiplier and divider values could be determined!");
					}
					mula = mul.unwrap() - 1;
					diva = div.unwrap() as u8;
		    	}

				pmc.ckgr_pllar.write( |w| {
					w.one().set_bit();
					unsafe { w.mula().bits(mula) };
					unsafe { w.diva().bits(diva) };
					unsafe { w.pllacount().bits(self.plla_conf.startup_cycles) }
				});
				while pmc.pmc_sr.read().locka().bit_is_clear() {
					//Wait until clock has stabilized
				}

				plla_freq = Hertz(mainck_freq.0 * (mula + 1) as u32 / (diva) as u32);
			}
		}
		

		// Upll configuration
		let upll_freq:Hertz;
		if !self.upll_conf.enable {
			pmc.ckgr_uckr.write( |w| w.upllen().clear_bit() );
			upll_freq = Hertz(0);
		} else {
			match self.upll_conf.src {
				UpllckSrcFreq::SRC12MHz => unsafe{ (*UTMI::ptr()).utmi_cktrim.write(|w| w.freq().xtal12()); }
				UpllckSrcFreq::SRC16MHz => unsafe{ (*UTMI::ptr()).utmi_cktrim.write(|w| w.freq().xtal16()); }
			}
			pmc.ckgr_uckr.write( |w| {
				unsafe { w.upllcount().bits(self.upll_conf.startup_cycles); }
				w.upllen().set_bit()
			});
			while pmc.pmc_sr.read().locku().bit_is_clear() {
				//Wait until clock has stabilized
			}
			upll_freq = MegaHertz(480).into();
		}

		// Master Clock configuration

		//set divider for uplldiv signal
		let uplldiv_freq : Hertz = match self.mck_conf.uplldiv {
				UpllDiv::Div1 => upll_freq,
				UpllDiv::Div2 => Hertz( upll_freq.0 / 2 )
		};
		pmc.pmc_mckr.modify( |_,w| {
			match self.mck_conf.uplldiv {
				UpllDiv::Div1 => {
					w.uplldiv2().clear_bit()
				}
				UpllDiv::Div2 => {
					w.uplldiv2().set_bit()
				}
			}
		});
		while pmc.pmc_sr.read().mckrdy().bit_is_clear() {
			//Wait for configuration to be applied
		}

		//set divider values
		let processor_freq:Hertz;
		let peripheral_freq:Hertz;
		let master_src_freq: Hertz = match self.mck_conf.src {
			MasterClockSrc::MAINCK => mainck_freq,
			MasterClockSrc::PLLACK => plla_freq,
			MasterClockSrc::UPLLCKDIV => uplldiv_freq,
			MasterClockSrc::SLCK => self.slck_conf.freq
		};
		pmc.pmc_mckr.modify(|_,w| {
			match self.mck_conf.pres {
				MasterPrescale::Pres1 => w.pres().clk_1(),
				MasterPrescale::Pres2 => w.pres().clk_2(),
				MasterPrescale::Pres3 => w.pres().clk_3(),
				MasterPrescale::Pres4 => w.pres().clk_4(),
				MasterPrescale::Pres8 => w.pres().clk_8(),
				MasterPrescale::Pres16 => w.pres().clk_16(),
				MasterPrescale::Pres32 => w.pres().clk_32(),
				MasterPrescale::Pres64 => w.pres().clk_64()
			};
			match self.mck_conf.mdiv {
				MasterDivider::Div1 => w.mdiv().eq_pck(),
				MasterDivider::Div2 => w.mdiv().pck_div2(),
				MasterDivider::Div3 => w.mdiv().pck_div3(),
				MasterDivider::Div4 => w.mdiv().pck_div4()
			}
		});
		processor_freq = Hertz( master_src_freq.0 / self.mck_conf.pres.to_value() );
		peripheral_freq = Hertz( processor_freq.0 / self.mck_conf.mdiv.to_value() );
		while pmc.pmc_sr.read().mckrdy().bit_is_clear() {
			//Wait for configuration to be applied
		}

		//set master clock src
		pmc.pmc_mckr.modify(|_,w| {
			match self.mck_conf.src {
				MasterClockSrc::MAINCK => w.css().main_clk(),
				MasterClockSrc::PLLACK => w.css().plla_clk(),
				MasterClockSrc::UPLLCKDIV => w.css().upll_clk(),
				MasterClockSrc::SLCK => w.css().slow_clk()
			}
		});
		while pmc.pmc_sr.read().mckrdy().bit_is_clear() {
			//Wait for configuration to be applied
		}

		Clocks {
			slck : self.slck_conf.freq,
			mainck : mainck_freq,
			plla : plla_freq,
			upll : upll_freq,
			uplldiv : uplldiv_freq,

			mck : peripheral_freq,
			fclk : processor_freq,
			sys_tick : Hertz(processor_freq.0 / 2),
			hclk : processor_freq,

		}
	}
}

/// Frozen clock frequencies
///
/// The existance of this value indicates that the clock configuration should no longer be changed
pub struct Clocks {
	/// Processor Clock frequency
	hclk : Hertz,

	/// SysTick frequency
	sys_tick : Hertz,

	/// Free Running Processor Clock frequency
	fclk : Hertz,

	/// Master Clock frequency
	mck: Hertz,

	/// Slow Clock frequency
	slck : Hertz,

	/// Main Clock frequency
	mainck : Hertz,

	/// Plla Clock frequency
	plla : Hertz,

	/// Uplldiv Clock frequency
	uplldiv : Hertz,

	///UPLL usb clock frequency
	upll : Hertz
}

impl Clocks {
	/// Returns Processor frequency
	pub fn hclk(&self) -> Hertz {
		self.hclk
	}

	/// Returns SysTick frequency
	pub fn sys_tick(&self) -> Hertz {
		self.sys_tick
	}

	/// Returns Free Running Processor Clock frequency
	pub fn fclk(&self) -> Hertz {
		self.fclk
	}

	/// Returns Master Clock frequency
	pub fn mck(&self) -> Hertz {
		self.mck
	}

	/// Returns Slow Clock frequency
	pub fn slck(&self) -> Hertz {
		self.slck
	}

	/// Returns Mainck frequency
	pub fn mainck(&self) -> Hertz {
		self.mainck
	}

	/// Returns Plla frequency
	pub fn plla(&self) -> Hertz {
		self.plla
	}

	/// Returns Uplldiv frequency
	pub fn uplldiv(&self) -> Hertz {
		self.uplldiv
	}

	/// Returns UPLL usb clock frequency
	pub fn upll(&self) -> Hertz {
		self.upll
	}
}
