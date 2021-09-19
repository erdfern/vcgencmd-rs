//! # Bindings for the RaspberryPi's vcgencmd cli utility

use std::num::{ParseFloatError, ParseIntError};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use subprocess::{Exec, PopenError, Redirection};

use bitpat::bitpat;

mod parsers;

#[derive(Debug)]
pub enum ExecutionError {
    Popen(PopenError),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
}

pub enum ClockSrc {
    Arm,
    Core,
    Dpi,
    Emmc,
    H264,
    Hdmi,
    Isp,
    Pixel,
    Pwm,
    Uart,
    V3d,
    Vec,
}

pub enum VoltSrc {
    Core,
    SdramC,
    SdramI,
    SdramP,
}

pub enum MemSrc {
    Arm,
    Gpu,
}

pub enum Src {
    Clock(ClockSrc),
    Mem(MemSrc),
    Volt(VoltSrc),
}

pub enum Cmd {
    GetMem,
    GetThrottled,
    MeasureClock,
    MeasureTemp,
    MeasureVolts,
}

/// This struct represents the possible information in a bit-pattern you would get
/// from the get_throttled command.
#[derive(Debug, Default, PartialOrd, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ThrottledStatus {
    pub arm_frequency_cap_occurred: bool,
    pub arm_frequency_capped: bool,
    pub currently_throttled: bool,
    pub soft_temp_limit_active: bool,
    pub soft_temp_limit_occurred: bool,
    pub throttling_occurred: bool,
    pub under_voltage: bool,
    pub under_voltage_occurred: bool,
}

impl ThrottledStatus {
    pub fn new(bit_pattern: isize) -> ThrottledStatus {
        interpret_bit_pattern(bit_pattern)
    }
}

/// Execute the given command and capture its std_output without modifying it
#[cfg(not(feature = "no-sudo"))]
pub fn exec_command(command: Cmd, src: Option<Src>) -> Result<String, PopenError> {
    // "vcgencmd" must be in PATH
    const VCGENCMD_INVOCATION: &str = "vcgencmd";

    let vcgencmd_output = Exec::cmd("sudo")
        .arg(VCGENCMD_INVOCATION)
        .arg(resolve_command(command))
        .arg(resolve_src(src).unwrap_or_default())
        .stdout(Redirection::Pipe)
        .capture()?
        .stdout_str();

    Ok(vcgencmd_output)
}

/// Execute the given command and capture its std_output without modifying it
#[cfg(feature = "no-sudo")]
pub fn exec_command(command: Cmd, src: Option<Src>) -> Result<String, PopenError> {
    // "vcgencmd" must be in PATH
    const VCGENCMD_INVOCATION: &str = "vcgencmd";

    let vcgencmd_output = Exec::cmd(VCGENCMD_INVOCATION)
        .arg(resolve_command(command))
        .arg(resolve_src(src).unwrap_or_default())
        .stdout(Redirection::Pipe)
        .capture()?
        .stdout_str();

    Ok(vcgencmd_output)
}

/// Measure the clock of the selected `ClockSrc`, returning the frequency as an isize
pub fn measure_clock(src: Src) -> Result<isize, ExecutionError> {
    let output = exec_command(Cmd::MeasureClock, Some(src)).map_err(ExecutionError::Popen)?;
    let frequency = parsers::frequency(&output).map_err(ExecutionError::ParseInt)?;

    Ok(frequency)
}

pub fn measure_volts(src: Src) -> Result<f64, ExecutionError> {
    let output = exec_command(Cmd::MeasureVolts, Some(src)).map_err(ExecutionError::Popen)?;
    let volts = parsers::volts(&output).map_err(ExecutionError::ParseFloat)?;

    Ok(volts)
}

pub fn measure_temp() -> Result<f64, ExecutionError> {
    let output = exec_command(Cmd::MeasureTemp, None).map_err(ExecutionError::Popen)?;
    let temperature = parsers::temp(&output).map_err(ExecutionError::ParseFloat)?;

    Ok(temperature)
}

pub fn get_mem(src: Src) -> Result<isize, ExecutionError> {
    let output = exec_command(Cmd::GetMem, Some(src)).map_err(ExecutionError::Popen)?;
    let mem = parsers::mem(&output).map_err(ExecutionError::ParseInt)?;

    Ok(mem)
}

pub fn get_throttled() -> Result<isize, ExecutionError> {
    let output = exec_command(Cmd::GetThrottled, None).map_err(ExecutionError::Popen)?;
    let bit_pattern = parsers::throttled(&output).map_err(ExecutionError::ParseInt)?;
    Ok(bit_pattern)
}

/// Interprets a bit pattern obtained from `get_throttled` in the following way:
/// ```txt
/// 111100000000000001010
/// ||||             ||||_ under-voltage
/// ||||             |||_ currently throttled
/// ||||             ||_ arm frequency capped
/// ||||             |_ soft temperature reached
/// ||||_ under-voltage has occurred since last reboot
/// |||_ throttling has occurred since last reboot
/// ||_ arm frequency capped has occurred since last reboot
/// |_ soft temperature reached since last reboot
/// ```
///
/// > Note: This interpretation might be false/outdated for different versions of vcgencmd...
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use vcgencmd::{interpret_bit_pattern, ThrottledStatus};
/// let throttle_status = interpret_bit_pattern(0b111100000000000001010_isize);
/// // or bit_pattern = get_throttle().unwrap();
/// // let throttle status = interpret_bit_pattern(bit_pattern);
/// assert_eq!(throttle_status,
///            ThrottledStatus {
///               arm_frequency_cap_occurred: true,
///               arm_frequency_capped: false,
///               currently_throttled: true,
///               soft_temp_limit_active: true,
///               soft_temp_limit_occurred: true,
///               throttling_occurred: true,
///               under_voltage: false,
///               under_voltage_occurred: true,
/// })
/// ```
pub fn interpret_bit_pattern(pattern: isize) -> ThrottledStatus {
    let soft_temp_limit_occurred = bitpat!(1 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _)(pattern);
    let arm_frequency_cap_occurred = bitpat!(_ 1 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _)(pattern);
    let throttling_occurred = bitpat!(_ _ 1 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _)(pattern);
    let under_voltage_occurred = bitpat!(_ _ _ 1 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _)(pattern);

    let soft_temp_limit_active = bitpat!(_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 1 _ _ _)(pattern);
    let arm_frequency_capped = bitpat!(_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 1 _ _)(pattern);
    let currently_throttled = bitpat!(_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 1 _)(pattern);
    let under_voltage = bitpat!(_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 1)(pattern);

    ThrottledStatus {
        arm_frequency_cap_occurred,
        arm_frequency_capped,
        currently_throttled,
        soft_temp_limit_active,
        soft_temp_limit_occurred,
        throttling_occurred,
        under_voltage,
        under_voltage_occurred,
    }
}

fn resolve_command(cmd: Cmd) -> String {
    match cmd {
        Cmd::GetMem => "get_mem",
        Cmd::GetThrottled => "get_throttled",
        Cmd::MeasureClock => "measure_clock",
        Cmd::MeasureTemp => "measure_temp",
        Cmd::MeasureVolts => "measure_volts",
    }
    .to_owned()
}

fn resolve_src(src: Option<Src>) -> Option<String> {
    // check for None
    let src = src.as_ref()?;

    match src {
        Src::Clock(ClockSrc::Arm) => Some("arm".to_owned()),
        Src::Clock(ClockSrc::Core) => Some("core".to_owned()),
        Src::Clock(ClockSrc::Dpi) => Some("dpi".to_owned()),
        Src::Clock(ClockSrc::Emmc) => Some("emmc".to_owned()),
        Src::Clock(ClockSrc::H264) => Some("h264".to_owned()),
        Src::Clock(ClockSrc::Hdmi) => Some("hdmi".to_owned()),
        Src::Clock(ClockSrc::Isp) => Some("isp".to_owned()),
        Src::Clock(ClockSrc::Pixel) => Some("pixel".to_owned()),
        Src::Clock(ClockSrc::Pwm) => Some("pwm".to_owned()),
        Src::Clock(ClockSrc::Uart) => Some("uart".to_owned()),
        Src::Clock(ClockSrc::V3d) => Some("v3d".to_owned()),
        Src::Clock(ClockSrc::Vec) => Some("vec".to_owned()),
        Src::Mem(MemSrc::Arm) => Some("arm".to_owned()),
        Src::Mem(MemSrc::Gpu) => Some("gpu".to_owned()),
        Src::Volt(VoltSrc::Core) => Some("core".to_owned()),
        Src::Volt(VoltSrc::SdramC) => Some("sdram_c".to_owned()),
        Src::Volt(VoltSrc::SdramI) => Some("sdram_i".to_owned()),
        Src::Volt(VoltSrc::SdramP) => Some("sdram_p".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_src() {
        assert_eq!(
            Some(String::from("arm")),
            resolve_src(Some(Src::Clock(ClockSrc::Arm)))
        );

        assert_eq!(None, resolve_src(None));
    }

    #[test]
    fn test_resolve_command() {
        assert_eq!("measure_temp", resolve_command(Cmd::MeasureTemp));
        assert_eq!("measure_clock", resolve_command(Cmd::MeasureClock));
    }

    #[test]
    fn test_throttled_status_methods() {
        let throttled_status = ThrottledStatus::new(0b111100000000000001010);
        assert_eq!(
            throttled_status,
            ThrottledStatus {
                arm_frequency_cap_occurred: true,
                arm_frequency_capped: false,
                currently_throttled: true,
                soft_temp_limit_active: true,
                soft_temp_limit_occurred: true,
                throttling_occurred: true,
                under_voltage: false,
                under_voltage_occurred: true,
            }
        )
    }

    #[test]
    fn test_interpret_bit_pattern() {
        let throttled_info = interpret_bit_pattern(0b111100000000000001010);
        assert_eq!(
            throttled_info,
            ThrottledStatus {
                arm_frequency_cap_occurred: true,
                arm_frequency_capped: false,
                currently_throttled: true,
                soft_temp_limit_active: true,
                soft_temp_limit_occurred: true,
                throttling_occurred: true,
                under_voltage: false,
                under_voltage_occurred: true,
            }
        );

        let throttled_info2 = interpret_bit_pattern(0b111100000000000001111);
        assert_eq!(
            throttled_info2,
            ThrottledStatus {
                arm_frequency_cap_occurred: true,
                arm_frequency_capped: true,
                currently_throttled: true,
                soft_temp_limit_active: true,
                soft_temp_limit_occurred: true,
                throttling_occurred: true,
                under_voltage: true,
                under_voltage_occurred: true,
            }
        )
    }

    #[cfg(target_arch = "arm")]
    #[test]
    fn test_exec_command() {
        let output = exec_command(Cmd::MeasureClock, Some(Src::Clock(ClockSrc::Core))).unwrap();
        dbg!(&output);
        assert!(output.contains("frequency"));
    }

    #[cfg(target_arch = "arm")]
    #[test]
    fn test_get_mem() {
        let output = get_mem(Src::Mem(MemSrc::Arm));
        dbg!(&output);
        debug_assert_eq!(output.is_ok(), true)
    }

    #[cfg(target_arch = "arm")]
    #[test]
    fn test_measure_temp() {
        let output = measure_temp();
        dbg!(&output);
        debug_assert_eq!(output.is_ok(), true)
    }

    #[cfg(target_arch = "arm")]
    #[test]
    fn test_measure_volts() {
        let output = measure_volts(Src::Volt(VoltSrc::Core));
        dbg!(&output);
        debug_assert_eq!(output.is_ok(), true)
    }

    #[cfg(target_arch = "arm")]
    #[test]
    fn test_measure_frequency() {
        let output = get_mem(Src::Mem(MemSrc::Arm));
        dbg!(&output);
        debug_assert_eq!(output.is_ok(), true)
    }
}
