# Bindings for RaspberryPi's vcgencmd utility

![Crates.io](https://img.shields.io/crates/v/vcgencmd)
[![Documentation](https://docs.rs/vcgencmd/badge.svg)](https://docs.rs/serde_rustler)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)

`vcgencmd` provides a way to interact with the vcgencmd utility included in Raspbian.

As of yet, not all vcgencmd commands have a binding. To see which commands are missing, take a look at PROGRESS.md in the projects repo. I will only actively add bindings for commands if I happen to need them.
If you need a specific command that's unimplemented, feel free to open an issue asking for it or submit it yourself.

## Installation

Install from [Crates.io](https://crates.io/crates/vcgencmd):

```toml
[dependencies]
vcgencmd = "0.2.*"
```

Serialization and deserialization for the few structs this crate contains are supported via a `serde_support` feature flag:
```toml
[dependencies]
vcgencmd = {version: "0.2.*", features = ["serde_support"]}
```

## Quick Start

```rust
// Import the various commands you want to use
use vcgencmd::{measure_temp, get_throttle, interpret_bit_pattern};

// You'll also want to import the `Src` enum, which holds all available sources
// for the different commands
use vcgendcmd::Src;

// Gives the current temperature as f64 in °C
let temp = measure_temp().unwrap();

// Measure the arm chips memory usage
let arm_mem = get_mem(Src::Mem(MemSrc::Arm)).unwrap();

// Measure the voltage at the video core
let volt_gpu = measure_volts(Src::Volt(VoltSrc::Core)).unwrap();

// Get a bit pattern which represents the throttled state of the system
let bit_pattern = get_throttle.unwrap();

// Get comprehensive, human readable info about the throttled state of the system
let throttle_status = interpret_bit_pattern(bit_pattern);
```
