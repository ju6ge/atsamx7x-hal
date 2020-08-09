//! External Bus Inteface pin wrapper

use core::marker::PhantomData;

pub trait EBIPins{}

pub struct ExternalBusInterface{
	pins : PhantomData<dyn EBIPins>
}

impl ExternalBusInterface {
	pub fn new(_pins : &dyn EBIPins) -> ExternalBusInterface {
		ExternalBusInterface{
			pins: PhantomData
		}
	}
}
