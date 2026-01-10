#![cfg_attr(not(test), no_std)]

mod bisync_gpio;
mod driver;
mod pin;

pub use bisync_gpio::{Level, Pull};
pub use driver::Sx1503;
pub use pin::Pins;

pub const ADDR: u8 = 0x20;

device_driver::create_device!(
    device_name: Device,
    manifest: "device.yaml"
);

/// The I2C wrapper interface to the driver
#[derive(Debug)]
pub struct DeviceInterface<I2C> {
    pub(crate) i2c: I2C,
}

impl<I2C> DeviceInterface<I2C> {
    pub(crate) const fn new(i2c: I2C) -> Self {
        Self { i2c }
    }
}

#[path = "."]
mod asynchronous {
    use bisync::asynchronous::*;
    use device_driver::AsyncRegisterInterface as RegisterInterface;
    use embedded_hal_async::{digital, i2c};

    mod gpio;
    mod input;
    mod ll;
    mod output;

    pub use input::*;
    pub use output::*;
}

pub use asynchronous::Input as InputAsync;
pub use asynchronous::Output as OutputAsync;

#[path = "."]
mod blocking {
    use bisync::synchronous::*;
    use device_driver::RegisterInterface;
    use embedded_hal::{digital, i2c};

    mod gpio;
    mod input;
    mod ll;
    mod output;

    pub use input::*;
    pub use output::*;
}

pub use blocking::Input;
pub use blocking::Output;
