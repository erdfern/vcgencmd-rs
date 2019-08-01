# Bindings for RaspberryPi's vcgencmd utility

<!-- [![Build Status](https://semaphoreci.com/api/v1/sunny-g/xdr/branches/master/badge.svg)](https://semaphoreci.com/sunny-g/xdr) -->
[![Crates.io](https://img.shields.io/crates/v/vcgencmd.svg)](https://crates.io/crates/vcgencmd)
[![Documentation](https://docs.rs/vcgencmd/badge.svg)](https://docs.rs/serde_rustler)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)

`vcgencmd` provides a way to interact with the vcgencmd utility included in Raspian.

As of yet, not all vcgencmd commands have a binding. To see which commands are missing, take a look at PROGRESS.md in the projects repo.
If you need a specific command that's unimplemented, feel free to open a pull request either asking  for it or submitting it yourself. 

## Installation

Install from [Crates.io](https://crates.io/crates/vcgencmd):

```toml
[dependencies]
vcgencmd = "0.2.0"
```

Serialization and deserialization for the few structs this crate contains are supported via a `serde` feature flag:
```toml
[dependencies]
vcgencmd = {version: "0.2.0", features = ["serde"]
```

## Quick Start

```rust
use vcgencmd::{measure_temp, get_throttle, interpret_bit_pattern};

// Gives the current temperature as f64 in °C
let temp = measure_temp();

let throttle_status = interpret_bit_pattern(get_throttle.unwrap());
```
