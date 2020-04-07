//! Serial communication

use core::fmt;
use core::marker::PhantomData;
use core::ptr;

use embedded_hal::prelude::*;
use embedded_hal::serial;

use crate::target_device::{UART0, UART1, UART2, UART3, UART4};
use crate::target_device::{USART0, USART1, USART2};
use crate::target_device::PMC;

use crate::gpio::{PeripheralCntr, PeriphA, PeriphB, PeriphC, PeriphD};
use crate::gpio::pioa::{PA4, PA5, PA6, PA9, PA10, PA21, PA23};
use crate::gpio::piob::{PB0, PB1, PB4, PB13};
use crate::gpio::piod::{PD3, PD15, PD16, PD17, PD18, PD19, PD25, PD26, PD28, PD30, PD31};
use crate::clock_gen::Clocks;

/// Serial error
#[derive(Debug)]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
    #[doc(hidden)]
    _Extensible,
}

/// Interrupt event
pub enum Event {
    /// New data has been received
    Rxne,
    /// New data can be sent
    Txe,
    /// Idle line state detected
    Idle,
}

pub mod config {
    use crate::time::Bps;
    use crate::time::U32Ext;

    pub enum WordLength {
        DataBits5,
        DataBits6,
        DataBits7,
        DataBits8,
    }

    pub enum Parity {
        ParityEven,
        ParityOdd,
        ParitySpace,
        ParityMark,
        ParityNone,
        ParityMultidrop
    }

    pub enum StopBits {
        #[doc = "1 stop bit"]
        STOP1,
        #[doc = "1.5 stop bits"]
        STOP1P5,
        #[doc = "2 stop bits"]
        STOP2,
    }

    pub struct UartConfig {
        pub baudrate: Bps,
        pub parity: Parity,
    }

    impl UartConfig {
        pub fn baudrate(mut self, baudrate: Bps) -> Self {
            self.baudrate = baudrate;
            self
        }

        pub fn parity_none(mut self) -> Self {
            self.parity = Parity::ParityNone;
            self
        }

        pub fn parity_even(mut self) -> Self {
            self.parity = Parity::ParityEven;
            self
        }

        pub fn parity_odd(mut self) -> Self {
            self.parity = Parity::ParityOdd;
            self
        }

		pub fn parity_space(mut self) -> Self {
			self.parity = Parity::ParitySpace;
			self
		}

		pub fn parity_mark(mut self) -> Self {
			self.parity = Parity::ParityMark;
			self
		}
    }

    #[derive(Debug)]
    pub struct InvalidConfig;

    impl Default for UartConfig {
        fn default() -> UartConfig {
            let baudrate = 19_200_u32.bps();
            UartConfig {
                baudrate,
                parity: Parity::ParityNone,
            }
        }
    }
}


pub trait Pins<USART> {}
pub trait PinTx<USART> {}
pub trait PinRx<USART> {}
pub trait PinCk<USART> {}

impl<USART, TX, RX> Pins<USART> for (TX, RX)
where
    TX: PinTx<USART>,
    RX: PinRx<USART>,
{
}

/// A filler type for when the Tx pin is unnecessary
pub struct NoTx;
/// A filler type for when the Rx pin is unnecessary
pub struct NoRx;
/// A filler type for when the Ck pin is unnecessary
pub struct NoCk;

macro_rules! usart_pins {
    ($($USARTX:ty: TX: [$($TX:ty),*] RX: [$($RX:ty),*] CK: [$($CK:ty),*])+) => {
        $(
            $(
                impl PinTx<$USARTX> for $TX {}
            )*
            $(
                impl PinRx<$USARTX> for $RX {}
            )*
            $(
                impl PinCk<$USARTX> for $CK {}
            )*
        )+
    }
}

macro_rules! uart_pins {
    ($($UARTX:ty: TX: [$($TX:ty),*] RX: [$($RX:ty),*])+) => {
        $(
            $(
                impl PinTx<$UARTX> for $TX {}
            )*
            $(
                impl PinRx<$UARTX> for $RX {}
            )*
        )+
    }
}

usart_pins! {
	USART0:
		TX : [
			PB1<PeripheralCntr<PeriphC>>,
			NoTx
		]
		RX : [
			PB0<PeripheralCntr<PeriphC>>,
			NoRx
		]
		CK : [
			PB13<PeripheralCntr<PeriphC>>,
			NoCk
		]
	USART1:
		TX : [
			PB4<PeripheralCntr<PeriphD>>,
			NoTx
		]
		RX : [
			PA21<PeripheralCntr<PeriphA>>,
			NoRx
		]
		CK : [
			PA23<PeripheralCntr<PeriphA>>,
			NoCk
		]
	USART2:
		TX : [
			PD16<PeripheralCntr<PeriphB>>,
			NoTx
		]
		RX : [
			PD15<PeripheralCntr<PeriphB>>,
			NoRx
		]
		CK : [
			PD17<PeripheralCntr<PeriphB>>,
			NoCk
		]
}

uart_pins! {
	UART0:
		TX : [
			PA10<PeripheralCntr<PeriphA>>,
			NoTx
		]
		RX: [
			PA9<PeripheralCntr<PeriphA>>,
			NoRx
		]
	UART1:
		TX : [
			PA4<PeripheralCntr<PeriphC>>,
			PA6<PeripheralCntr<PeriphC>>,
			PD26<PeripheralCntr<PeriphD>>,
			NoTx
		]
		RX: [
			PA5<PeripheralCntr<PeriphC>>,
			NoRx
		]
	UART2:
		TX : [
			PD26<PeripheralCntr<PeriphC>>,
			NoTx
		]
		RX: [
			PD25<PeripheralCntr<PeriphC>>,
			NoRx
		]
	UART3:
		TX : [
			PD30<PeripheralCntr<PeriphA>>,
			PD31<PeripheralCntr<PeriphB>>,
			NoTx
		]
		RX: [
			PD28<PeripheralCntr<PeriphA>>,
			NoRx
		]
	UART4:
		TX : [
			PD3<PeripheralCntr<PeriphC>>,
			PD19<PeripheralCntr<PeriphC>>,
			NoTx
		]
		RX: [
			PD18<PeripheralCntr<PeriphC>>,
			NoRx
		]
}

/// Serial abstraction
pub struct Serial<USART, PINS> {
	usart: USART,
	pins: PINS,
}

/// Serial receiver
pub struct Rx<USART> {
	_usart: PhantomData<USART>,
}

/// Serial transmitter
pub struct Tx<USART> {
	_usart: PhantomData<USART>,
}

pub trait SerialExt<USART> {
	fn uart<PINS>(
		self,
		pins: PINS,
		config: config::UartConfig,
		pmc: &mut PMC,
	) -> Result<Serial<USART, PINS>, config::InvalidConfig>
	where
		PINS: Pins<USART>;

	//Todo the USART module is much more complex on this device -> TODO for future
}


macro_rules! uart_hal {
	($( $UARTX:ident: (
			$uartX:ident,
			$en_reg:ident,
			$perid:ident
		),
	)+) => {
		$(
			/// Configures a UART peripheral to provide serial communication
			impl<PINS> Serial<$UARTX, PINS> {
				pub fn $uartX(
					uart: $UARTX,
					pins: PINS,
					config: config::UartConfig,
					clocks: &Clocks,
					pmc: &mut PMC,
				) -> Result<Self, config::InvalidConfig>
				where
					PINS: Pins<$UARTX>,
				{
					use self::config::*;

					//enable peripheral clock in pmc
					pmc.$en_reg.write(|w| w.$perid().set_bit() );

					//reset peripheral
					uart.uart_cr.write(|w| {
						w.rstrx().set_bit();
						w.rsttx().set_bit();
						w.rxdis().set_bit();
						w.txdis().set_bit();
						w.rststa().set_bit()
					});

					//calc correct baudrate div
					let clk_div = clocks.mck().0 / (16 * config.baudrate.0);
					uart.uart_brgr.write(|w| unsafe{w.bits(clk_div)} );

					//set mode
					uart.uart_mr.write(|w| {
						//normal mode
						w.chmode().bits(0);

						//peripheral clk as src
						w.brsrcck().clear_bit();

						//parity
						unsafe {w.par().bits( match config.parity {
							Parity::ParityEven => 0,
							Parity::ParityOdd => 1,
							Parity::ParitySpace => 2,
							Parity::ParityMark => 3,
							Parity::ParityNone => 4,
							//multidrop not available for this peripheral -> default to no parity
							Parity::ParityMultidrop => 4,
						})}
					});


					//enable receiver and transmitter
					uart.uart_cr.write(|w| {
						w.txen().set_bit();
						w.rxen().set_bit()
					});

					Ok(Serial{usart: uart, pins})
				}

				/// Splits the `Serial` abstraction into a transmitter and a receiver half
				pub fn split(self) -> (Tx<$UARTX>, Rx<$UARTX>) {

					(Tx {
						_usart: PhantomData,
					},
					Rx {
						_usart: PhantomData,
					},)
				}

				/// Releases the USART peripheral and associated pins
				pub fn release(self) -> ($UARTX, PINS) {
					(self.usart, self.pins)
				}
			}

			impl<PINS> serial::Read<u8> for Serial<$UARTX, PINS> {
				type Error = Error;

				fn read(&mut self) -> nb::Result<u8, Error> {
					let mut rx: Rx<$UARTX> = Rx {
						_usart: PhantomData,
					};
					rx.read()
				}
			}

			impl serial::Read<u8> for Rx<$UARTX> {
				type Error = Error;

				fn read(&mut self) -> nb::Result<u8, Error> {
					// NOTE(unsafe) atomic read with no side effects
					let sr = unsafe { (*$UARTX::ptr()).uart_sr.read() };

					// Any error requires the dr to be read to clear
					if sr.pare().bit_is_set()
						|| sr.frame().bit_is_set()
						|| sr.ovre().bit_is_set()
					{
						unsafe { (*$UARTX::ptr()).uart_rhr.read() };
					}

					Err(if sr.pare().bit_is_set() {
						nb::Error::Other(Error::Parity)
					} else if sr.frame().bit_is_set() {
						nb::Error::Other(Error::Framing)
					} else if sr.ovre().bit_is_set() {
						nb::Error::Other(Error::Overrun)
					} else if sr.rxrdy().bit_is_set() {
						// NOTE(read_volatile) see `write_volatile` below
						return Ok(unsafe { ptr::read_volatile(&(*$UARTX::ptr()).uart_rhr as *const _ as *const _) });
					} else {
						nb::Error::WouldBlock
					})
				}
			}

			impl<PINS> serial::Write<u8> for Serial<$UARTX, PINS> {
				type Error = Error;

				fn flush(&mut self) -> nb::Result<(), Self::Error> {
					let mut tx: Tx<$UARTX> = Tx {
						_usart: PhantomData,
					};
					tx.flush()
				}

				fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
					let mut tx: Tx<$UARTX> = Tx {
						_usart: PhantomData,
					};
					tx.write(byte)
				}
			}

			impl serial::Write<u8> for Tx<$UARTX> {
				type Error = Error;

				fn flush(&mut self) -> nb::Result<(), Self::Error> {
					// NOTE(unsafe) atomic read with no side effects
					let sr = unsafe { (*$UARTX::ptr()).uart_sr.read() };

					if sr.txempty().bit_is_set() {
						Ok(())
					} else {
						Err(nb::Error::WouldBlock)
					}
				}

				fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
					// NOTE(unsafe) atomic read with no side effects
					let sr = unsafe { (*$UARTX::ptr()).uart_sr.read() };

					if sr.txrdy().bit_is_set() {
						// NOTE(unsafe) atomic write to stateless register
						// NOTE(write_volatile) 8-bit write that's not possible through the svd2rust API
						unsafe { ptr::write_volatile(&(*$UARTX::ptr()).uart_thr as *const _ as *mut _, byte) }
						Ok(())
					} else {
						Err(nb::Error::WouldBlock)
					}
				}
			}
		)+
	}
}

uart_hal! {
	UART0 : (uart0, pmc_pcer0, pid7),
	UART1 : (uart1, pmc_pcer0, pid8),
	UART2 : (uart2, pmc_pcer1, pid44),
	UART3 : (uart3, pmc_pcer1, pid45),
	UART4 : (uart4, pmc_pcer1, pid46),
}

impl<USART, PINS> fmt::Write for Serial<USART, PINS>
	where
	    Serial<USART, PINS>: crate::hal::serial::Write<u8>,
    {
		fn write_str(&mut self, s: &str) -> fmt::Result {
			let _ = s
				.as_bytes()
                .into_iter()
	            .map(|c| nb::block!(self.write(*c)))
                .last();
                Ok(())
        }
    }

impl<USART> fmt::Write for Tx<USART>
    where
        Tx<USART>: crate::hal::serial::Write<u8>,
    {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            let _ = s
                .as_bytes()
                .into_iter()
                .map(|c| nb::block!(self.write(*c)))
                .last();
                Ok(())
        }
    }
