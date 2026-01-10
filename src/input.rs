use crate::{Device, DeviceInterface, Pull, pin::Pin};

use super::{
    bisync,
    digital::{ErrorType, InputPin},
    gpio::Gpio,
    i2c::I2c,
    ll::DeviceError,
};

pub struct Input<P, I> {
    gpio: Gpio<P, I>,
}

impl<P, I> Input<P, I>
where
    P: Pin,
    I: I2c,
{
    #[bisync]
    pub async fn new(pin: P, interface: I, pull: Pull) -> Result<Self, DeviceError<I::Error>> {
        let device = Device::new(DeviceInterface::new(interface));
        let mut gpio = Gpio::new(pin, device);

        gpio.set_as_input().await?;
        gpio.set_pull(pull).await?;

        Ok(Self { gpio })
    }
}

impl<P, I> ErrorType for Input<P, I>
where
    P: Pin,
    I: I2c,
{
    type Error = DeviceError<I::Error>;
}

#[bisync]
impl<P, I> InputPin for Input<P, I>
where
    P: Pin,
    I: I2c,
{
    async fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.gpio.is_high().await
    }

    async fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.gpio.is_low().await
    }
}
