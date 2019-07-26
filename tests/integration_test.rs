extern crate vcgencmd;

use vcgencmd::ClockSrc;

#[test]
fn test_measure_clock() {
    let output = vcgencmd::measure_clock(vcgencmd::Src::ClockSrc(ClockSrc::Arm)).unwrap();
    dbg!(&output);
    assert!(!output.is_empty());
}
