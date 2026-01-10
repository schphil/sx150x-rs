mod common;

use common::*;

#[test]
fn test_output_io2_initial_level_low_set_high() {
    let expectations = [
        // configure pin as output
        // - read direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DIR_A], vec![0x00]),
        // - write direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DIR_A]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_A], vec![0x00]),
        // - write data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DATA_A]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_A], vec![0x00]),
        // - write data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   1   0   0   = 0x04
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DATA_A]),
        I2cTransaction::write(ADDR, vec![0x04]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   1   0   0   = 0x04
        I2cTransaction::write_read(ADDR, vec![REG_DATA_A], vec![0x04]),
    ];

    let pins = init_sx1503();
    let mut i2c = I2cMock::new(&expectations);
    let mut output = Output::new(pins.pin2, i2c.clone(), Level::Low).unwrap();

    output.set_high().unwrap();
    assert!(output.is_set_high().unwrap());

    i2c.done();
}

#[test]
fn test_output_io5_initial_high() {
    let expectations = [
        // configure pin as output
        // - read direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DIR_A], vec![0x00]),
        // - write direction bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DIR_A]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_A], vec![0x00]),
        // - write data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   1   0   0   0   0   0   = 0x20
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DATA_A]),
        I2cTransaction::write(ADDR, vec![0x20]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   1   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_A], vec![0x20]),
        // - write data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DATA_A]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank A
        // - IO7 IO6 IO5 IO4 IO3 IO2 IO1 IO0
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_A], vec![0x00]),
    ];

    let pins = init_sx1503();
    let mut i2c = I2cMock::new(&expectations);
    let mut output = Output::new(pins.pin5, i2c.clone(), Level::High).unwrap();

    output.set_low().unwrap();
    assert!(output.is_set_low().unwrap());

    i2c.done();
}

#[test]
fn test_output_io8_initial_level_low_set_high() {
    let expectations = [
        // configure pin as output
        // - read direction bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DIR_B], vec![0x00]),
        // - write direction bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DIR_B]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_B], vec![0x00]),
        // - write data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DATA_B]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_B], vec![0x00]),
        // - write data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   1   = 0x01
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DATA_B]),
        I2cTransaction::write(ADDR, vec![0x01]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   1   = 0x01
        I2cTransaction::write_read(ADDR, vec![REG_DATA_B], vec![0x01]),
    ];

    let pins = init_sx1503();
    let mut i2c = I2cMock::new(&expectations);
    let mut output = Output::new(pins.pin8, i2c.clone(), Level::Low).unwrap();

    output.set_high().unwrap();
    assert!(output.is_set_high().unwrap());

    i2c.done();
}

#[test]
fn test_output_io15_initial_high() {
    let expectations = [
        // configure pin as output
        // - read direction bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DIR_B], vec![0x00]),
        // - write direction bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DIR_B]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_B], vec![0x00]),
        // - write data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 1   0   0   0   0   0   0   0   = 0x80
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DATA_B]),
        I2cTransaction::write(ADDR, vec![0x80]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_B], vec![0x80]),
        // - write data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::transaction_start(ADDR),
        I2cTransaction::write(ADDR, vec![REG_DATA_B]),
        I2cTransaction::write(ADDR, vec![0x00]),
        I2cTransaction::transaction_end(ADDR),
        // - read data bank B
        // - I15 I14 I13 I12 I11 I10 I09 IO8
        // - 0   0   0   0   0   0   0   0   = 0x00
        I2cTransaction::write_read(ADDR, vec![REG_DATA_B], vec![0x00]),
    ];

    let pins = init_sx1503();
    let mut i2c = I2cMock::new(&expectations);
    let mut output = Output::new(pins.pin15, i2c.clone(), Level::High).unwrap();

    output.set_low().unwrap();
    assert!(output.is_set_low().unwrap());

    i2c.done();
}
