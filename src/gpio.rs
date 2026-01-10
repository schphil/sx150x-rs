use crate::{Device, DeviceInterface, Pull, pin::Bank, pin::Pin};

use super::{RegisterInterface, bisync, i2c::I2c, ll::DeviceError, only_async, only_sync};

pub(crate) struct Gpio<P, I> {
    pin: P,
    device: Device<DeviceInterface<I>>,
}

include!("bisync_helpers.rs");

#[bisync]
impl<P, I> Gpio<P, I>
where
    P: Pin,
    I: I2c,
{
    pub(crate) fn new(pin: P, device: Device<DeviceInterface<I>>) -> Self {
        Gpio { pin, device }
    }

    #[inline]
    pub async fn set_pull(&mut self, pull: Pull) -> Result<(), DeviceError<I::Error>> {
        let (pu, pd) = match pull {
            Pull::Up => (true, false),
            Pull::Down => (false, true),
            Pull::None => (false, false),
        };

        let pin = self.pin.pin();

        match self.pin.bank() {
            Bank::BankA => {
                let mut op = self.device.pull_up_a();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    if pu {
                        val |= 1 << pin;
                    } else {
                        val &= !(1 << pin);
                    }

                    *reg = [val].into();
                })
                .await?;

                let mut op = self.device.pull_down_a();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    if pd {
                        val |= 1 << pin;
                    } else {
                        val &= !(1 << pin);
                    }

                    *reg = [val].into();
                })
                .await?;
            }
            Bank::BankB => {
                let mut op = self.device.pull_up_b();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    if pu {
                        val |= 1 << pin;
                    } else {
                        val &= !(1 << pin);
                    }

                    *reg = [val].into();
                })
                .await?;

                let mut op = self.device.pull_down_b();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    if pd {
                        val |= 1 << pin;
                    } else {
                        val &= !(1 << pin);
                    }

                    *reg = [val].into();
                })
                .await?;
            }
        }

        Ok(())
    }

    #[inline]
    pub async fn set_as_input(&mut self) -> Result<(), DeviceError<I::Error>> {
        let pin = self.pin.pin();

        match self.pin.bank() {
            Bank::BankA => {
                let mut op = self.device.dir_a();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    val |= 1 << pin;

                    *reg = [val].into();
                })
                .await?;
            }
            Bank::BankB => {
                let mut op = self.device.dir_b();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    val |= 1 << pin;

                    *reg = [val].into();
                })
                .await?;
            }
        }

        Ok(())
    }

    #[inline]
    pub async fn set_as_output(&mut self) -> Result<(), DeviceError<I::Error>> {
        let pin = self.pin.pin();

        match self.pin.bank() {
            Bank::BankA => {
                let mut op = self.device.dir_a();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    val &= !(1 << pin);

                    *reg = [val].into();
                })
                .await?;
            }
            Bank::BankB => {
                let mut op = self.device.dir_b();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    val &= !(1 << pin);

                    *reg = [val].into();
                })
                .await?;
            }
        }

        Ok(())
    }

    #[inline]
    pub async fn is_high(&mut self) -> Result<bool, DeviceError<I::Error>> {
        Ok(!self.is_low().await?)
    }

    #[inline]
    pub async fn is_low(&mut self) -> Result<bool, DeviceError<I::Error>> {
        let pin = self.pin.pin();

        match self.pin.bank() {
            Bank::BankA => {
                let mut op = self.device.data_a();
                let reg = read_internal(&mut op).await?;

                let bytes: [u8; 1] = reg.into();
                let val = bytes[0];

                Ok((val & (1 << pin)) == 0)
            }
            Bank::BankB => {
                let mut op = self.device.data_b();
                let reg = read_internal(&mut op).await?;

                let bytes: [u8; 1] = reg.into();
                let val = bytes[0];

                Ok((val & (1 << pin)) == 0)
            }
        }
    }

    #[inline]
    pub async fn set_high(&mut self) -> Result<(), DeviceError<I::Error>> {
        let pin = self.pin.pin();

        match self.pin.bank() {
            Bank::BankA => {
                let mut op = self.device.data_a();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    val |= 1 << pin;

                    *reg = [val].into();
                })
                .await?;
            }
            Bank::BankB => {
                let mut op = self.device.data_b();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    val |= 1 << pin;

                    *reg = [val].into();
                })
                .await?;
            }
        }

        Ok(())
    }

    #[inline]
    pub async fn set_low(&mut self) -> Result<(), DeviceError<I::Error>> {
        let pin = self.pin.pin();

        match self.pin.bank() {
            Bank::BankA => {
                let mut op = self.device.data_a();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    val &= !(1 << pin);

                    *reg = [val].into();
                })
                .await?;
            }
            Bank::BankB => {
                let mut op = self.device.data_b();

                modify_internal(&mut op, |reg| {
                    let bytes: [u8; 1] = (*reg).into();
                    let mut val = bytes[0];

                    val &= !(1 << pin);

                    *reg = [val].into();
                })
                .await?;
            }
        }

        Ok(())
    }
}
