//! This crate contains bindings for the RaspberryPi's vcgencmd cli tool

use subprocess::{PopenError, Exec, Redirection};

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

enum Sources {
    KVoltSources(KVoltSources),
}

/// Execute the given command and capture its std_output
pub fn exec_command(command: &str) -> String {
    Exec::cmd(command).stdout(Redirection::Pipe).capture()?.stdout_str()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
