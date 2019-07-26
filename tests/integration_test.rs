extern crate vcgencmd;

use vcgencmd::ClockArgs;

#[test]
fn test_measure_clock() {
    let output = vcgencmd::measure_clock(vcgencmd::Args::ClockArgs(ClockArgs::Arm)).unwrap();
    dbg!(&output);
    assert!(!output.is_empty());
}
