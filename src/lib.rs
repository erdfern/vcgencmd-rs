//! This crate contains bindings for the RaspberryPi's vcgencmd cli tool

use subprocess::{Exec, PopenError, Redirection};

pub enum ClockArgs {
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

pub enum Args {
    ClockArgs(ClockArgs),
}

/// Execute the given command and capture its std_output
fn exec_command(command: &str, arg: &str) -> Result<String, PopenError> {
    let vcgencmd_output = Exec::cmd("vcgencmd")
        .arg(arg)
        .stdout(Redirection::Pipe)
        .capture()?
        .stdout_str();

    Ok(vcgencmd_output)
}

fn resolve_arg(arg: Args) -> String {
    let arg = match arg {
        Args::ClockArgs(ClockArgs::Arm) => "arm",
        Args::ClockArgs(ClockArgs::Core) => "core",
        Args::ClockArgs(ClockArgs::Dpi) => "dpi",
        Args::ClockArgs(ClockArgs::Emmc) => "emmc",
        Args::ClockArgs(ClockArgs::H264) => "h264",
        Args::ClockArgs(ClockArgs::Hdmi) => "hdmi",
        Args::ClockArgs(ClockArgs::Isp) => "isp",
        Args::ClockArgs(ClockArgs::Pixel) => "pixel",
        Args::ClockArgs(ClockArgs::Pwm) => "pwm",
        Args::ClockArgs(ClockArgs::Uart) => "uart",
        Args::ClockArgs(ClockArgs::V3d) => "v3d",
        Args::ClockArgs(ClockArgs::Vec) => "vec",
    }
    .to_owned();

    arg
}

pub fn measure_clock(arg: Args) -> Result<String, PopenError> {
    let output = exec_command("measure_clock", &resolve_arg(arg))?;

    Ok(output)
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
        let ls_output = exec_command("ls", "-a").unwrap();
        dbg!(&ls_output);
        assert!(!ls_output.is_empty());
    }
}
