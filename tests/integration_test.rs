extern crate vcgencmd;

#[cfg(target_arch = "arm")]
#[test]
fn test_measure_clock() {
    let output = vcgencmd::measure_clock(vcgencmd::Src::Clock(ClockSrc::Arm)).unwrap();
    dbg!(&output);
    // Idiotic
}
