//! This crate contains bindings for the RaspberryPi's vcgencmd cli tool
use subprocess::{Exec, PopenError, Redirection};
use std::num::ParseIntError;

// "vcgencmd" must be in PATH
const VCGENCMD_INVOCATION: &str = "vcgencmd";

#[derive(Debug)]
pub enum ExecutionError {
    Popen(PopenError),
    Parse(ParseIntError),
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

pub enum Src {
    ClockSrc(ClockSrc),
}

pub enum Cmd {
    MeasureClock,
    MeasureTemp,
}

/// Execute the given command and capture its std_output without modifying it
pub fn exec_command(command: Cmd, arg: Src) -> Result<String, PopenError> {
    let vcgencmd_output = Exec::cmd(VCGENCMD_INVOCATION)
        .arg(resolve_command(command))
        .arg(resolve_src(arg))
        .stdout(Redirection::Pipe)
        .capture()?
        .stdout_str();

    Ok(vcgencmd_output)
}

/// Measure the clock of the selected `ClockSrc`, returning the frequency as isize
pub fn measure_clock(src: Src) -> Result<isize, ExecutionError> {
    let output = exec_command(Cmd::MeasureClock, src).map_err(ExecutionError::Popen)?;
    let frequency = output.split("=").collect::<Vec<_>>()[1].parse::<isize>().map_err(ExecutionError::Parse)?;
    Ok(frequency)
}

fn resolve_command(cmd: Cmd) -> String {
    let command = match cmd {
        Cmd::MeasureClock => "measure_clock",
        Cmd::MeasureTemp => "measure_temp",
    }
        .to_owned();

    command
}

fn resolve_src(src: Src) -> String {
    let source = match src {
        Src::ClockSrc(ClockSrc::Arm) => "arm",
        Src::ClockSrc(ClockSrc::Core) => "core",
        Src::ClockSrc(ClockSrc::Dpi) => "dpi",
        Src::ClockSrc(ClockSrc::Emmc) => "emmc",
        Src::ClockSrc(ClockSrc::H264) => "h264",
        Src::ClockSrc(ClockSrc::Hdmi) => "hdmi",
        Src::ClockSrc(ClockSrc::Isp) => "isp",
        Src::ClockSrc(ClockSrc::Pixel) => "pixel",
        Src::ClockSrc(ClockSrc::Pwm) => "pwm",
        Src::ClockSrc(ClockSrc::Uart) => "uart",
        Src::ClockSrc(ClockSrc::V3d) => "v3d",
        Src::ClockSrc(ClockSrc::Vec) => "vec",
    }
        .to_owned();

    source
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exec_command_works() {
        let ls_output = exec_command(Cmd::MeasureClock, Src::ClockSrc(ClockSrc::Core))
            .unwrap();
        dbg!(&ls_output);
        assert!(!ls_output.is_empty());
    }
}
