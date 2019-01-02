//! Build script for `stm32-nucleo`
//!
//! The purpose of this script is to add some additional configuration features based on the
//! specific device chosen. Additionally, a simple memory map is generated for the device.

#![allow(non_camel_case_types)]

use std::io::Write;
use std::{env, fmt};

use strum::IntoEnumIterator;
use strum_macros::*;

fn main() {
    let feature = get_feature();
    let (form, family, map) = feature.expand();

    export_cfg(&form.to_string());
    export_cfg(&family.to_string());
    let map_str = map.build(&feature.to_string());

    let mut file = std::fs::File::create("memory.x")
        .expect("failed to create memory.x");
    file.write_all(map_str.as_bytes()).unwrap();
}

/// Currently supported devices.
///
/// Unfortunately, this must be kept in sync with the features defined in Cargo.toml.
#[derive(Display, EnumIter, Debug)]
enum Feature {
    NUCLEO_F401RE,
    NUCLEO_F411RE,
    NUCLEO_F412RG,
}

/// Determine which feature flag was enabled based on env variables set by Cargo
fn get_feature() -> Feature {
    for feature in Feature::iter() {
        let env_str = format!("CARGO_FEATURE_{}", feature.to_string());
        if env::var(&env_str).is_ok() {
            return feature;
        }
    }

    panic!("Please provide a valid feature flag.");
}

/// Export as a cfg feature
fn export_cfg(string: &str) {
    println!("cargo:rustc-cfg={}", string);
}

impl Feature {
    /// Return information about the device feature
    fn expand(&self) -> (Form, Family, MemoryMap) {
        match self {
            Feature::NUCLEO_F401RE => {
                let map = MemoryMap {
                    flash: (0x02000000, "64K".to_string()),
                    ram: (0x02000000, "256K".to_string()),
                };
                (Form::Nucleo64, Family::Stm32F4xx, map)
            }
            Feature::NUCLEO_F411RE => {
                let map = MemoryMap {
                    flash: (0x08000000, "512K".to_string()),
                    ram: (0x20000000, "128K".to_string()),
                };
                (Form::Nucleo64, Family::Stm32F4xx, map)
            }
            Feature::NUCLEO_F412RG => {
                let map = MemoryMap {
                    flash: (0x08000000, "1M".to_string()),
                    ram: (0x20000000, "256K".to_string()),
                };
                (Form::Nucleo64, Family::Stm32F4xx, map)
            }
        }
    }
}

/// Possible board form factors for STM32 Nucleo
#[derive(Debug)]
enum Form {
    Nucleo32,
    Nucleo64,
    Nucleo128,
}

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s.to_lowercase())
    }
}

/// STM32 device family
#[derive(Debug)]
enum Family {
    Stm32F0xx,
    Stm32F2xx,
    Stm32F3xx,
    Stm32F4xx,
    Stm32F7xx,
    Stm32H7xx,
    Stm32L0xx,
    Stm32L1xx,
    Stm32L4xx,
}

impl fmt::Display for Family {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s.to_lowercase())
    }
}

/// A very simple memory map
struct MemoryMap {
    flash: (u32, String),
    ram: (u32, String),
}

impl MemoryMap {
    /// Produce memory.x as a string
    fn build(self, device: &str) -> String {
        format!(
"/* File autogenerated by `build.rs` for the {} */
MEMORY
{{
    /* NOTE K = KiBi = 1024 bytes */
    FLASH : ORIGIN = 0x{:08x}, LENGTH = {}
    RAM : ORIGIN = 0x{:08x}, LENGTH = {}
}}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);",
        device, self.flash.0, self.flash.1, self.ram.0, self.ram.1)
    }
}