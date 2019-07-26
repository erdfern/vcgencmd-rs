//! This crate contains bindings for the RaspberryPi's vcgencmd cli tool

enum KVoltSources {
    Arm,
    Core,
    H264,
    Isp,
    V3d,
    Uart,
    Pwm,
    Emmc,
    Pixel,
    Vec,
    Hdmi,
    Dpi,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
