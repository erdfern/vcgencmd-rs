//! This crate contains bindings for the RaspberryPi's vcgencmd cli tool
use std::num::{ParseFloatError, ParseIntError};

use subprocess::{Exec, PopenError, Redirection};

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
    MeasureClock,
    MeasureTemp,
    MeasureVolts,
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
    let frequency = parsers::frequency(&output)
        .map_err(ExecutionError::ParseInt)?;

    Ok(frequency)
}

pub fn measure_volts(src: Src) -> Result<f64, ExecutionError> {
    let output = exec_command(Cmd::MeasureVolts, Some(src)).map_err(ExecutionError::Popen)?;
    let volts = parsers::volts(&output).map_err(ExecutionError::ParseFloat)?;

    Ok(volts)
}

pub fn measure_temp() -> Result<f64, ExecutionError> {
    let output = exec_command(Cmd::MeasureTemp, None).map_err(ExecutionError::Popen)?;
    let temperature = parsers::temp(&output)
        .map_err(ExecutionError::ParseFloat)?;

    Ok(temperature)
}

pub fn get_mem(src: Src) -> Result<isize, ExecutionError> {
    let output = exec_command(Cmd::GetMem, Some(src)).map_err(ExecutionError::Popen)?;
    let mem = parsers::mem(&output)
        .map_err(ExecutionError::ParseInt)?;

    Ok(mem)
}

fn resolve_command(cmd: Cmd) -> String {
    let command = match cmd {
        Cmd::GetMem => "get_mem",
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
        _ => "",
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
