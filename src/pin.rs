#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub(crate) enum Bank {
    BankA,
    BankB,
}

pub(crate) trait SealedPin {
    /// returns the pin number within a bank
    fn pin(&self) -> u8;

    /// returns the bank of this pin
    fn bank(&self) -> Bank;
}

pub trait Pin: SealedPin {}

macro_rules! impl_pin {
    ($name:ident, $bank:expr, $pin_num:expr) => {
        pub struct $name;

        impl SealedPin for $name {
            fn pin(&self) -> u8 {
                $pin_num % 8
            }

            fn bank(&self) -> Bank {
                $bank
            }
        }

        impl Pin for $name {}
    };
}

impl_pin!(Pin0, Bank::BankA, 0);
impl_pin!(Pin1, Bank::BankA, 1);
impl_pin!(Pin2, Bank::BankA, 2);
impl_pin!(Pin3, Bank::BankA, 3);
impl_pin!(Pin4, Bank::BankA, 4);
impl_pin!(Pin5, Bank::BankA, 5);
impl_pin!(Pin6, Bank::BankA, 6);
impl_pin!(Pin7, Bank::BankA, 7);
impl_pin!(Pin8, Bank::BankB, 8);
impl_pin!(Pin9, Bank::BankB, 9);
impl_pin!(Pin10, Bank::BankB, 10);
impl_pin!(Pin11, Bank::BankB, 11);
impl_pin!(Pin12, Bank::BankB, 12);
impl_pin!(Pin13, Bank::BankB, 13);
impl_pin!(Pin14, Bank::BankB, 14);
impl_pin!(Pin15, Bank::BankB, 15);

pub struct Pins {
    pub pin0: Pin0,
    pub pin1: Pin1,
    pub pin2: Pin2,
    pub pin3: Pin3,
    pub pin4: Pin4,
    pub pin5: Pin5,
    pub pin6: Pin6,
    pub pin7: Pin7,
    pub pin8: Pin8,
    pub pin9: Pin9,
    pub pin10: Pin10,
    pub pin11: Pin11,
    pub pin12: Pin12,
    pub pin13: Pin13,
    pub pin14: Pin14,
    pub pin15: Pin15,
}

#[allow(clippy::new_without_default)]
impl Pins {
    pub fn new() -> Self {
        Pins {
            pin0: Pin0,
            pin1: Pin1,
            pin2: Pin2,
            pin3: Pin3,
            pin4: Pin4,
            pin5: Pin5,
            pin6: Pin6,
            pin7: Pin7,
            pin8: Pin8,
            pin9: Pin9,
            pin10: Pin10,
            pin11: Pin11,
            pin12: Pin12,
            pin13: Pin13,
            pin14: Pin14,
            pin15: Pin15,
        }
    }
}
