#![no_std]
#![no_main]
#![feature(asm)]

extern crate panic_halt;
extern crate atsamx7x_hal as hal;

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

use hal::target_device;
use hal::gpio::*;
use hal::clock_gen::{Clocks, MasterClockConfig, SlckConfig, MainckConfig, PllackConfig, UpllckConfig, SystemClockConfig, MasterDivider, MasterPrescale};
use hal::serial::{config, Serial};
use hal::time::*;
use hal::delay::Delay;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

use embedded_hal::digital::v2::ToggleableOutputPin;

use core::fmt::Write;

#[entry]
fn main() -> ! {
	let cortex_p = cortex_m::Peripherals::take().unwrap();
	let peripherals = target_device::Peripherals::take().unwrap();

	let wdt = &peripherals.WDT;
	wdt.wdt_mr.write( |w| w.wddis().set_bit() );

	let mut pmc = peripherals.PMC;
	let mut supc = peripherals.SUPC;

	peripherals.EFC.eefc_fmr.write( |w| {
		unsafe {w.fws().bits(5);}
		w.cloe().set_bit();
		w.scod().clear_bit()
	});

	let clocks:Clocks = SystemClockConfig{
		slck_conf : SlckConfig::default(),
		mainck_conf : MainckConfig::default().use_crystal(hal::time::MegaHertz(12).into()).disable_rc(),
		plla_conf : PllackConfig::default().from_divider(1, 49).startup_cycles(100),
		upll_conf : UpllckConfig::default().enable(),
		mck_conf :MasterClockConfig::default().src_pllack().from_divider(MasterPrescale::Pres2, MasterDivider::Div2)
	}.freeze(&mut pmc, &mut supc);

	let mut delay = Delay::new(cortex_p.SYST, &clocks);

	let pioc = peripherals.PIOC.split(&mut pmc);
	let mut pin0 = pioc.p19.into_open_drain_output();
	let mut pin1 = pioc.p10.into_open_drain_output();

	let pioa = peripherals.PIOA.split(&mut pmc);
	let tx = pioa.p10.into_peripheral_a();
	let rx = pioa.p9.into_peripheral_a();

	let mut serial = Serial::uart0(
		peripherals.UART0,
		(tx, rx),
		config::UartConfig::default().baudrate(115_200.bps()),
		&clocks,
		&mut pmc
	).unwrap();

	//blink
	loop {
		pin0.toggle().unwrap();
		pin1.toggle().unwrap();
		delay.delay_ms(500 as u32);
		writeln!(serial, "Hello, world!\r").unwrap();
	}
}
