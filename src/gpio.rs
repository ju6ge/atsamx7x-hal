//! Pin configuration

use core::marker::PhantomData;

use crate::target_device::PMC;

pub trait GpioExt {
	///The pins the PIO is split into
	type Parts;

	///Splits the PIO block into independant pins
	fn split(self, pmc: &mut PMC) -> Self::Parts;
}
/// Input mode (type state)
pub struct Input<MODE> {
	_mode : PhantomData<MODE>,
}

// Floating input (type state)
pub struct Floating;
// Pulled down input (type state)
pub struct PullDown;
//Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
	_mode : PhantomData<MODE>,
}

// OpenDrain output (type state)
pub struct OpenDrain;

/// Output mode (type state)
pub struct PeripheralCntr<MODE> {
	_mode : PhantomData<MODE>,
}

// Peripheral multiplexer device 0 (type state)
pub struct PeriphA;
// Peripheral multiplexer device 1 (type state)
pub struct PeriphB;
// Peripheral multiplexer device 2 (type state)
pub struct PeriphC;
// Peripheral multiplexer device 3 (type state)
pub struct PeriphD;

/**
Macro to generate Traits for every PIO controller on the device
 PIOX  : PIO device
 piox  :
 perid : peripheral id
 PXx   :
*/
macro_rules! gpio {
	(
		$PIOX:ident, $piox:ident, $pioy:ident, $perid:ident, $PXx:ident, [
			$($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
	]) => {
		/// PIO
		pub mod $piox {
			use core::convert::Infallible;
			use core::marker::PhantomData;

			use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, InputPin};
			use embedded_hal::digital::v2::toggleable;
			use crate::target_device::{$PIOX};

			use crate::target_device::PMC;
			#[allow(unused_imports)]
			use super::{
				PeriphA, PeriphB, PeriphC, PeriphD, OpenDrain, Floating, PullUp, PullDown, Input, Output, GpioExt, PeripheralCntr
			};



			/// PIO parts
			pub struct Parts {
				$(
					pub $pxi: $PXi<$MODE>,
				)+
			}

			impl GpioExt for $PIOX {
				type Parts = Parts;

				fn split(self, pmc: &mut PMC) -> Parts {
					pmc.pmc_pcer0.write( |w| w.$perid().set_bit());

					Parts {
						$(
							$pxi: $PXi { _mode: PhantomData },
						)+
					}
				}
			}


			$(
				pub struct $PXi<MODE> {
					_mode: PhantomData<MODE>,
				}

				impl<MODE> $PXi<MODE> {
					pub fn into_pmd0(
						self,
					) -> $PXi<PeripheralCntr<PeriphA>>{
						unsafe {
							&(*$PIOX::ptr()).pio_abcdsr[0].write(|w| { w.$pxi().clear_bit() });
							&(*$PIOX::ptr()).pio_abcdsr[1].write(|w| { w.$pxi().clear_bit() });
							&(*$PIOX::ptr()).pio_pdr.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

					pub fn into_pmd1(
						self,
					) -> $PXi<PeripheralCntr<PeriphB>>{
						unsafe {
							&(*$PIOX::ptr()).pio_abcdsr[0].write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_abcdsr[1].write(|w| { w.$pxi().clear_bit() });
							&(*$PIOX::ptr()).pio_pdr.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}


					pub fn into_pmd2(
						self,
					) -> $PXi<PeripheralCntr<PeriphC>>{
						unsafe {
							&(*$PIOX::ptr()).pio_abcdsr[0].write(|w| { w.$pxi().clear_bit() });
							&(*$PIOX::ptr()).pio_abcdsr[1].write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_pdr.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}


					pub fn into_pmd3(
						self,
					) -> $PXi<PeripheralCntr<PeriphD>>{
						unsafe {
							&(*$PIOX::ptr()).pio_abcdsr[0].write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_abcdsr[1].write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_pdr.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

					pub fn into_open_drain_output(
						self,
					) -> $PXi<Output<OpenDrain>> {
						unsafe {
							&(*$PIOX::ptr()).pio_per.write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_oer.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

					pub fn into_floating_input(
						self,
					) -> $PXi<Input<Floating>> {
						unsafe {
							&(*$PIOX::ptr()).pio_per.write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_odr.write(|w| { w.$pxi().set_bit() });

							//disable possible pull up/down
							&(*$PIOX::ptr()).pio_ppddr.write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_pudr.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

					pub fn into_pull_down_input(
						self,
					) -> $PXi<Input<PullDown>> {
						unsafe {
							&(*$PIOX::ptr()).pio_per.write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_odr.write(|w| { w.$pxi().set_bit() });

							&(*$PIOX::ptr()).pio_ppder.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

					pub fn into_pull_up_input(
						self,
					) -> $PXi<Input<PullUp>> {
						unsafe {
							&(*$PIOX::ptr()).pio_per.write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_odr.write(|w| { w.$pxi().set_bit() });

							&(*$PIOX::ptr()).pio_puer.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

				}

				impl<MODE> OutputPin for $PXi<Output<MODE>> {
					type Error = Infallible;

					fn set_high(&mut self) -> Result<(), Self::Error> {
						//NOTE (unsafe) atomic write to a stateless register
						Ok(unsafe{ (*$PIOX::ptr()).pio_sodr.write(|w| w.bits(1 << $i ))})
					}
					fn set_low(&mut self) -> Result<(), Self::Error> {
						//NOTE (unsafe) atomic write to a stateless register
						Ok(unsafe{ (*$PIOX::ptr()).pio_codr.write(|w| w.bits(1 << $i ))})
					}
				}

				impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
					fn is_set_high(&self) -> Result<bool, Self::Error> {
						//NOTE (unsafe) atomic read to a stateless register
						Ok(unsafe{ (*$PIOX::ptr()).pio_odsr.read().$pxi().bit_is_set() } )
					}
					fn is_set_low(&self) -> Result<bool, Self::Error> {
						//NOTE (unsafe) atomic write to a stateless register
						Ok(unsafe{ (*$PIOX::ptr()).pio_odsr.read().$pxi().bit_is_clear() })
					}
				}

				impl<MODE> toggleable::Default for $PXi<Output<MODE>> {}

				impl<MODE> InputPin for $PXi<Input<MODE>> {
					type Error = Infallible;

					fn is_high(&self) -> Result<bool, Self::Error> {
						// NOTE(unsafe) atomic read with no side effects
						Ok(unsafe { (*$PIOX::ptr()).pio_pdsr.read().$pxi().bit_is_set() })
					}

					fn is_low(&self) -> Result<bool, Self::Error> {
						// NOTE(unsafe) atomic read with no side effects
						Ok(unsafe { (*$PIOX::ptr()).pio_pdsr.read().$pxi().bit_is_clear() })
					}
				}
			)+
		}
	}
}
gpio!(
	PIOA, pioa, pioa, pid10, Px, [
		PA0: (p0, 0, Input<Floating>),
		PA1: (p1, 1, Input<Floating>),
		PA2: (p2, 2, Input<Floating>),
		PA3: (p3, 3, Input<Floating>),
		PA4: (p4, 4, Input<Floating>),
		PA5: (p5, 5, Input<Floating>),
		PA6: (p6, 6, Input<Floating>),
		PA7: (p7, 7, Input<Floating>),
		PA8: (p8, 8, Input<Floating>),
		PA9: (p9, 9, Input<Floating>),
		PA10: (p10, 10, Input<Floating>),
		PA11: (p11, 11, Input<Floating>),
		PA12: (p12, 12, Input<Floating>),
		PA13: (p13, 13, Input<Floating>),
		PA14: (p14, 14, Input<Floating>),
		PA15: (p15, 15, Input<Floating>),
		PA16: (p16, 16, Input<Floating>),
		PA17: (p17, 17, Input<Floating>),
		PA18: (p18, 18, Input<Floating>),
		PA19: (p19, 19, Input<Floating>),
		PA20: (p20, 20, Input<Floating>),
		PA21: (p21, 21, Input<Floating>),
		PA22: (p22, 22, Input<Floating>),
		PA23: (p23, 23, Input<Floating>),
		PA24: (p24, 24, Input<Floating>),
		PA25: (p25, 25, Input<Floating>),
		PA26: (p26, 26, Input<Floating>),
		PA27: (p27, 27, Input<Floating>),
		PA28: (p28, 28, Input<Floating>),
		PA29: (p29, 29, Input<Floating>),
		PA30: (p30, 30, Input<Floating>),
		PA31: (p31, 31, Input<Floating>),
	]);

gpio!(
	PIOB, piob, pioa, pid11, Px, [
		PB0: (p0, 0, Input<Floating>),
		PB1: (p1, 1, Input<Floating>),
		PB2: (p2, 2, Input<Floating>),
		PB3: (p3, 3, Input<Floating>),
		PB4: (p4, 4, Input<Floating>),
		PB5: (p5, 5, Input<Floating>),
		PB6: (p6, 6, Input<Floating>),
		PB7: (p7, 7, Input<Floating>),
		PB8: (p8, 8, Input<Floating>),
		PB9: (p9, 9, Input<Floating>),
		PB10: (p10, 10, Input<Floating>),
		PB11: (p11, 11, Input<Floating>),
		PB12: (p12, 12, Input<Floating>),
		PB13: (p13, 13, Input<Floating>),
		PB14: (p14, 14, Input<Floating>),
		PB15: (p15, 15, Input<Floating>),
		PB16: (p16, 16, Input<Floating>),
		PB17: (p17, 17, Input<Floating>),
		PB18: (p18, 18, Input<Floating>),
		PB19: (p19, 19, Input<Floating>),
		PB20: (p20, 20, Input<Floating>),
		PB21: (p21, 21, Input<Floating>),
		PB22: (p22, 22, Input<Floating>),
		PB23: (p23, 23, Input<Floating>),
		PB24: (p24, 24, Input<Floating>),
		PB25: (p25, 25, Input<Floating>),
		PB26: (p26, 26, Input<Floating>),
		PB27: (p27, 27, Input<Floating>),
		PB28: (p28, 28, Input<Floating>),
		PB29: (p29, 29, Input<Floating>),
		PB30: (p30, 30, Input<Floating>),
		PB31: (p31, 31, Input<Floating>),
	]);

gpio!(
	PIOC, pioc, pioa, pid12, Px, [
		PC0: (p0, 0, Input<Floating>),
		PC1: (p1, 1, Input<Floating>),
		PC2: (p2, 2, Input<Floating>),
		PC3: (p3, 3, Input<Floating>),
		PC4: (p4, 4, Input<Floating>),
		PC5: (p5, 5, Input<Floating>),
		PC6: (p6, 6, Input<Floating>),
		PC7: (p7, 7, Input<Floating>),
		PC8: (p8, 8, Input<Floating>),
		PC9: (p9, 9, Input<Floating>),
		PC10: (p10, 10, Input<Floating>),
		PC11: (p11, 11, Input<Floating>),
		PC12: (p12, 12, Input<Floating>),
		PC13: (p13, 13, Input<Floating>),
		PC14: (p14, 14, Input<Floating>),
		PC15: (p15, 15, Input<Floating>),
		PC16: (p16, 16, Input<Floating>),
		PC17: (p17, 17, Input<Floating>),
		PC18: (p18, 18, Input<Floating>),
		PC19: (p19, 19, Input<Floating>),
		PC20: (p20, 20, Input<Floating>),
		PC21: (p21, 21, Input<Floating>),
		PC22: (p22, 22, Input<Floating>),
		PC23: (p23, 23, Input<Floating>),
		PC24: (p24, 24, Input<Floating>),
		PC25: (p25, 25, Input<Floating>),
		PC26: (p26, 26, Input<Floating>),
		PC27: (p27, 27, Input<Floating>),
		PC28: (p28, 28, Input<Floating>),
		PC29: (p29, 29, Input<Floating>),
		PC30: (p30, 30, Input<Floating>),
		PC31: (p31, 31, Input<Floating>),
	]);

gpio!(
	PIOD, piod, pioa, pid16, Px, [
		PD0: (p0, 0, Input<Floating>),
		PD1: (p1, 1, Input<Floating>),
		PD2: (p2, 2, Input<Floating>),
		PD3: (p3, 3, Input<Floating>),
		PD4: (p4, 4, Input<Floating>),
		PD5: (p5, 5, Input<Floating>),
		PD6: (p6, 6, Input<Floating>),
		PD7: (p7, 7, Input<Floating>),
		PD8: (p8, 8, Input<Floating>),
		PD9: (p9, 9, Input<Floating>),
		PD10: (p10, 10, Input<Floating>),
		PD11: (p11, 11, Input<Floating>),
		PD12: (p12, 12, Input<Floating>),
		PD13: (p13, 13, Input<Floating>),
		PD14: (p14, 14, Input<Floating>),
		PD15: (p15, 15, Input<Floating>),
		PD16: (p16, 16, Input<Floating>),
		PD17: (p17, 17, Input<Floating>),
		PD18: (p18, 18, Input<Floating>),
		PD19: (p19, 19, Input<Floating>),
		PD20: (p20, 20, Input<Floating>),
		PD21: (p21, 21, Input<Floating>),
		PD22: (p22, 22, Input<Floating>),
		PD23: (p23, 23, Input<Floating>),
		PD24: (p24, 24, Input<Floating>),
		PD25: (p25, 25, Input<Floating>),
		PD26: (p26, 26, Input<Floating>),
		PD27: (p27, 27, Input<Floating>),
		PD28: (p28, 28, Input<Floating>),
		PD29: (p29, 29, Input<Floating>),
		PD30: (p30, 30, Input<Floating>),
		PD31: (p31, 31, Input<Floating>),
	]);

gpio!(
	PIOE, pioe, pioa, pid17, Px, [
		PE0: (p0, 0, Input<Floating>),
		PE1: (p1, 1, Input<Floating>),
		PE2: (p2, 2, Input<Floating>),
		PE3: (p3, 3, Input<Floating>),
		PE4: (p4, 4, Input<Floating>),
		PE5: (p5, 5, Input<Floating>),
		PE6: (p6, 6, Input<Floating>),
		PE7: (p7, 7, Input<Floating>),
		PE8: (p8, 8, Input<Floating>),
		PE9: (p9, 9, Input<Floating>),
		PE10: (p10, 10, Input<Floating>),
		PE11: (p11, 11, Input<Floating>),
		PE12: (p12, 12, Input<Floating>),
		PE13: (p13, 13, Input<Floating>),
		PE14: (p14, 14, Input<Floating>),
		PE15: (p15, 15, Input<Floating>),
		PE16: (p16, 16, Input<Floating>),
		PE17: (p17, 17, Input<Floating>),
		PE18: (p18, 18, Input<Floating>),
		PE19: (p19, 19, Input<Floating>),
		PE20: (p20, 20, Input<Floating>),
		PE21: (p21, 21, Input<Floating>),
		PE22: (p22, 22, Input<Floating>),
		PE23: (p23, 23, Input<Floating>),
		PE24: (p24, 24, Input<Floating>),
		PE25: (p25, 25, Input<Floating>),
		PE26: (p26, 26, Input<Floating>),
		PE27: (p27, 27, Input<Floating>),
		PE28: (p28, 28, Input<Floating>),
		PE29: (p29, 29, Input<Floating>),
		PE30: (p30, 30, Input<Floating>),
		PE31: (p31, 31, Input<Floating>),
	]);
