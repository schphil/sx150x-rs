use thiserror::Error;

use crate::pin::Pins;

static TAKEN: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

#[derive(Debug, Error)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Sx1503Error<I2cErr> {
    #[error("I2C error")]
    I2c(I2cErr),
    #[error("Pins already taken")]
    PinsAlreadyTaken,
}

pub struct Sx1503 {}

impl Sx1503 {
    pub fn take_pins() -> Result<Pins, Sx1503Error<()>> {
        if TAKEN.load(core::sync::atomic::Ordering::SeqCst) {
            Err(Sx1503Error::PinsAlreadyTaken)
        } else {
            TAKEN.store(true, core::sync::atomic::Ordering::SeqCst);
            let pins = Pins::new();
            Ok(pins)
        }
    }

    /// # Safety
    ///
    /// You must ensure that you're only using one instance of the pins at a time.
    pub unsafe fn steal() -> Pins {
        Pins::new()
    }
}
