pub const ADDR: u8 = 0x20;
pub const REG_DATA_B: u8 = 0x00;
pub const REG_DATA_A: u8 = 0x01;
pub const REG_DIR_B: u8 = 0x02;
pub const REG_DIR_A: u8 = 0x03;
pub const REG_PULL_UP_B: u8 = 0x04;
pub const REG_PULL_UP_A: u8 = 0x05;
pub const REG_PULL_DOWN_B: u8 = 0x06;
pub const REG_PULL_DOWN_A: u8 = 0x07;

pub use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
pub use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
pub use sx150x::{Input, InputAsync, Level, Output, OutputAsync, Pins, Pull, Sx1503};

pub fn init_sx1503() -> Pins {
    unsafe { Sx1503::steal() }
}
