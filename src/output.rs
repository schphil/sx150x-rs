use crate::{Device, DeviceInterface, Level, pin::Pin};

use super::{
    bisync,
    digital::{ErrorType, OutputPin, StatefulOutputPin},
    gpio::Gpio,
    i2c::I2c,
    ll::DeviceError,
};

pub struct Output<P, I> {
    gpio: Gpio<P, I>,
}

#[bisync]
impl<P, I> Output<P, I>
where
    P: Pin,
    I: I2c,
{
    pub async fn new(
        pin: P,
        interface: I,
        initial_output: Level,
    ) -> Result<Self, DeviceError<I::Error>> {
        let device = Device::new(DeviceInterface::new(interface));
        let mut gpio = Gpio::new(pin, device);

        gpio.set_as_output().await?;

        match initial_output {
            Level::High => {
                gpio.set_high().await?;
            }
            Level::Low => {
                gpio.set_low().await?;
            }
        }

        Ok(Self { gpio })
    }
}

impl<P, I> ErrorType for Output<P, I>
where
    P: Pin,
    I: I2c,
{
    type Error = DeviceError<I::Error>;
}

#[bisync]
impl<P, I> OutputPin for Output<P, I>
where
    P: Pin,
    I: I2c,
{
    async fn set_high(&mut self) -> Result<(), Self::Error> {
        self.gpio.set_high().await
    }

    async fn set_low(&mut self) -> Result<(), Self::Error> {
        self.gpio.set_low().await
    }
}

#[bisync]
impl<P, I> StatefulOutputPin for Output<P, I>
where
    P: Pin,
    I: I2c,
{
    async fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        self.gpio.is_high().await
    }

    async fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        self.gpio.is_low().await
    }
}
