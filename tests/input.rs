mod common;

use common::*;

#[test]
fn test_input_io2_configure_pull_up() {
    let expectations = [
        // configure pin as input
        // - read direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DIR_A], vec![0x00]),
        // - write direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   1   0   0   = 0x04
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DIR_A]),
        I2cTransaction::write(ADDR, vec![0x04]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-up bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_UP_A], vec![0x00]),
        // - write pull-up bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   1   0   0   = 0x04
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_UP_A]),
        I2cTransaction::write(ADDR, vec![0x04]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-down bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_DOWN_A], vec![0x00]),
        // - write pull-down bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_DOWN_A]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
    ];

    let pins = init_sx1503();
    let mut i2c = I2cMock::new(&expectations);
    let _input = Input::new(pins.pin2, i2c.clone(), Pull::Up).unwrap();

    i2c.done();
}

#[test]
fn test_input_io5_configure_pull_down() {
    let expectations = [
        // configure pin as input
        // - read direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DIR_A], vec![0x00]),
        // - write direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   1   0   0   0   0   0   = 0x20
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DIR_A]),
        I2cTransaction::write(ADDR, vec![0x20]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-up bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_UP_A], vec![0x00]),
        // - write pull-up bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_UP_A]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-down bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_DOWN_A], vec![0x00]),
        // - write pull-down bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   1   0   0   0   0   0   = 0x20
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_DOWN_A]),
        I2cTransaction::write(ADDR, vec![0x20]),
        I2cTransaction::transaction_end(ADDR),
    ];

    let pins = init_sx1503();
    let mut i2c = I2cMock::new(&expectations);
    let _input = Input::new(pins.pin5, i2c.clone(), Pull::Down).unwrap();

    i2c.done();
}

#[test]
fn test_input_io0_low() {
    let expectations = [
        // configure pin as input
        // - read direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DIR_A], vec![0x00]),
        // - write direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   1   = 0x01
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DIR_A]),
        I2cTransaction::write(ADDR, vec![0x01]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-up bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_UP_A], vec![0x00]),
        // - write pull-up bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_UP_A]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-down bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_DOWN_A], vec![0x00]),
        // - write pull-down bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_DOWN_A]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // read input level
        // - read data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_A], vec![0x00]),
    ];

    let pins = init_sx1503();
    let mut i2c = I2cMock::new(&expectations);
    let mut input = Input::new(pins.pin0, i2c.clone(), Pull::None).unwrap();

    assert!(input.is_low().unwrap());

    i2c.done();
}

#[test]
fn test_input_io3_high() {
    let expectations = [
        // configure pin as input
        // - read direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DIR_A], vec![0x00]),
        // - write direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   1   0   0   0   = 0x08
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DIR_A]),
        I2cTransaction::write(ADDR, vec![0x08]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-up bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_UP_A], vec![0x00]),
        // - write pull-up bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_UP_A]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-down bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_DOWN_A], vec![0x00]),
        // - write pull-down bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_DOWN_A]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // read input level
        // - read data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   1   0   0   0   = 0x08
        I2cTransaction::write_read(ADDR, vec![REG_DATA_A], vec![0x08]),
    ];

    let pins = init_sx1503();
    let mut i2c = I2cMock::new(&expectations);
    let mut input = Input::new(pins.pin3, i2c.clone(), Pull::None).unwrap();

    assert!(input.is_high().unwrap());

    i2c.done();
}

#[test]
fn test_input_io8_low() {
    let expectations = [
        // configure pin as input
        // - read direction bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DIR_B], vec![0x00]),
        // - write direction bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   1   = 0x01
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DIR_B]),
        I2cTransaction::write(ADDR, vec![0x01]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-up bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_UP_B], vec![0x00]),
        // - write pull-up bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_UP_B]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-down bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_DOWN_B], vec![0x00]),
        // - write pull-down bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_DOWN_B]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // read input level
        // - read data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_B], vec![0x00]),
    ];

    let pins = init_sx1503();
    let mut i2c = I2cMock::new(&expectations);
    let mut input = Input::new(pins.pin8, i2c.clone(), Pull::None).unwrap();

    assert!(input.is_low().unwrap());

    i2c.done();
}

#[test]
fn test_input_io15_high() {
    let expectations = [
        // configure pin as input
        // - read direction bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DIR_B], vec![0x00]),
        // - write direction bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 1   0   0   0   0   0   0   0   = 0x80
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DIR_B]),
        I2cTransaction::write(ADDR, vec![0x80]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-up bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_UP_B], vec![0x00]),
        // - write pull-up bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_UP_B]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read pull-down bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_PULL_DOWN_B], vec![0x00]),
        // - write pull-down bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_PULL_DOWN_B]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // read input level
        // - read data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 1   0   0   0   0   0   0   0   = 0x80
        I2cTransaction::write_read(ADDR, vec![REG_DATA_B], vec![0x80]),
    ];

    let pins = init_sx1503();
    let mut i2c = I2cMock::new(&expectations);
    let mut input = Input::new(pins.pin15, i2c.clone(), Pull::None).unwrap();

    assert!(input.is_high().unwrap());

    i2c.done();
}
