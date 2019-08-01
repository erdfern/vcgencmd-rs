//! # Bindings for the RaspberryPi's vcgencmd cli utility

use std::num::{ParseFloatError, ParseIntError};

use subprocess::{Exec, PopenError, Redirection};

use bitpat::bitpat;

mod parsers;

// "vcgencmd" must be in PATH
const VCGENCMD_INVOCATION: &str = "vcgencmd";

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
#[cfg(feature = "serde")]
#[derive(Debug, Default, PartialOrd, PartialEq)]
pub struct ThrottledStatus {
    arm_frequency_cap_occurred: bool,
    arm_frequency_capped: bool,
    currently_throttled: bool,
    soft_temp_limit_active: bool,
    soft_temp_limit_occurred: bool,
    throttling_occurred: bool,
    under_voltage: bool,
    under_voltage_occurred: bool,
}

/// Execute the given command and capture its std_output without modifying it
pub fn exec_command(command: Cmd, src: Option<Src>) -> Result<String, PopenError> {
    if let None = src {
        let vcgencmd_output = Exec::cmd(VCGENCMD_INVOCATION)
            .arg(resolve_command(command))
            .stdout(Redirection::Pipe)
            .capture()?
            .stdout_str();

        return Ok(vcgencmd_output);
    };

    let vcgencmd_output = Exec::cmd(VCGENCMD_INVOCATION)
        .arg(resolve_command(command))
        .arg(resolve_src(src.unwrap()))
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
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use vcgencmd::{interpret_bit_pattern, ThrottledStatus};
/// let throttle_status = interpret_bit_pattern(0b111100000000000001010_isize);
/// // or bit_pattern = get_throttle().unwrap();
/// // let throttle status = interpret_bit_pattern(bit_pattern);
/// assert_eq!(throttle_info,
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
    let command = match cmd {
        Cmd::GetMem => "get_mem",
        Cmd::GetThrottled => "get_throttled",
        Cmd::MeasureClock => "measure_clock",
        Cmd::MeasureTemp => "measure_temp",
        Cmd::MeasureVolts => "measure_volts",
    }
    .to_owned();

    command
}

fn resolve_src(src: Src) -> String {
    let source = match src {
        Src::Clock(ClockSrc::Arm) => "arm",
        Src::Clock(ClockSrc::Core) => "core",
        Src::Clock(ClockSrc::Dpi) => "dpi",
        Src::Clock(ClockSrc::Emmc) => "emmc",
        Src::Clock(ClockSrc::H264) => "h264",
        Src::Clock(ClockSrc::Hdmi) => "hdmi",
        Src::Clock(ClockSrc::Isp) => "isp",
        Src::Clock(ClockSrc::Pixel) => "pixel",
        Src::Clock(ClockSrc::Pwm) => "pwm",
        Src::Clock(ClockSrc::Uart) => "uart",
        Src::Clock(ClockSrc::V3d) => "v3d",
        Src::Clock(ClockSrc::Vec) => "vec",
        Src::Mem(MemSrc::Arm) => "arm",
        Src::Mem(MemSrc::Gpu) => "gpu",
        Src::Volt(VoltSrc::Core) => "core",
        Src::Volt(VoltSrc::SdramC) => "sdram_c",
        Src::Volt(VoltSrc::SdramI) => "sdram_i",
        Src::Volt(VoltSrc::SdramP) => "sdram_p",
    }
    .to_owned();

    source
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_src() {
        assert_eq!("arm", resolve_src(Src::Clock(ClockSrc::Arm)));
    }

    #[test]
    fn test_resolve_command() {
        assert_eq!("measure_temp", resolve_command(Cmd::MeasureTemp));
        assert_eq!("measure_clock", resolve_command(Cmd::MeasureClock));
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
        assert!(!output.is_empty());
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
