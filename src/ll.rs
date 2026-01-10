use core::fmt;

use crate::{ADDR, DeviceInterface};

use super::{
    RegisterInterface, bisync, digital,
    i2c::{I2c, Operation},
};

#[bisync]
impl<I2C: I2c> RegisterInterface for DeviceInterface<I2C> {
    type Error = DeviceError<I2C::Error>;
    type AddressType = u8;

    /// ``` text
    /// Master: ST SAD+W     RA     OD     SP
    /// Sx150x:          SAK    SAK    SAK
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `SAD+W` = slave address followed by bit 0 to indicate writing
    /// - `SAK` = slave acknowledge
    /// - `RA` = register address byte
    /// - `OD` = output data byte
    /// - `SP` = stop condition
    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        Ok(self
            .i2c
            .transaction(
                ADDR,
                &mut [Operation::Write(&[address]), Operation::Write(data)],
            )
            .await?)
    }

    /// ``` text
    /// Master: ST SAD+W     RA     SR SAD+R        NMAK SP
    /// Sx150x:          SAK    SAK          SAK ID
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `SAD+W` = slave address followed by bit 0 to indicate writing
    /// - `SAK` = slave acknowledge
    /// - `RA` = register address byte
    /// - `SR` = repeated start condition
    /// - `SAD+R` = slave address followed by bit 1 to indicate reading
    /// - `ID` = input data byte
    /// - `MAK` = master acknowledge
    /// - `NMAK` = master no acknowledge
    /// - `SP` = stop condition
    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(self.i2c.write_read(ADDR, &[address], data).await?)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct DeviceError<I2C>(pub I2C);

impl<I2C> fmt::Debug for DeviceError<I2C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DeviceError(..)")
    }
}

impl<I2C> From<I2C> for DeviceError<I2C> {
    fn from(value: I2C) -> Self {
        Self(value)
    }
}

impl<I2C> core::ops::Deref for DeviceError<I2C> {
    type Target = I2C;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I2C> core::ops::DerefMut for DeviceError<I2C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I2C> digital::Error for DeviceError<I2C> {
    fn kind(&self) -> digital::ErrorKind {
        digital::ErrorKind::Other
    }
}
