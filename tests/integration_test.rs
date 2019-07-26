extern crate vcgencmd;

use vcgencmd::ClockSrc;

#[test]
fn test_measure_clock() {
    let output = vcgencmd::measure_clock(vcgencmd::Src::Clock(ClockSrc::Arm)).unwrap();
    dbg!(&output);
    // Idiotic
}
