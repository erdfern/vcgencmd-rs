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
use vcgencmd::{measure_temp, get_throttle, interpret_bit_pattern};

// Gives the current temperature as f64 in °C
let temp = measure_temp();

let throttle_status = interpret_bit_pattern(get_throttle.unwrap());
```
