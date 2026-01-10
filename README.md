# SX150x 4/8/16 Channel Low Voltage GPIO driver

A `no_std` driver for the SX150x family of I²C GPIO expanders, providing both synchronous and asynchronous APIs. Built with type-safe register definitions using the `device-driver` crate and full compatibility with [`embedded-hal`] traits.

## Usage

**Blocking:**
```rust
use sx150x::{Sx1503, Input, Pull};
use embedded_hal_async::i2c::I2c;

let mut i2c = I2c::new(...);
let pins =  Sx1503::take_pins().unwrap();
let mut input = Input::new(pins.pin0, i2c, Pull::Up);
let is_high = input.is_high()?;
```

**Async:**
```rust
use sx150x::{Sx1503, OutputAsync, Level};
use embedded_hal_async::i2c::I2c;

let mut i2c = I2c::new(...);
let pins =  Sx1503::take_pins().unwrap();
let mut output = OutputAsync::new(pins.pin8, i2c, Level::Low);
output.set_high().await?;
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
