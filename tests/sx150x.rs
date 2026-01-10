#[test]
fn test_pins_already_taken() {
    assert!(sx150x::Sx1503::take_pins().is_ok());
    assert!(sx150x::Sx1503::take_pins().is_err());
}
