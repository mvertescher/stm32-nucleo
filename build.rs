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
    // F0
    NUCLEO_F030R8,
    NUCLEO_F042K6,
    NUCLEO_F070RB,

    // F4
    NUCLEO_F401RE,
    NUCLEO_F411RE,
    NUCLEO_F412RG,
    NUCLEO_F412ZG,
    NUCLEO_F429ZI,

    // F7
    NUCLEO_F722ZE,
    NUCLEO_F746ZG,
    NUCLEO_F756ZG,
    NUCLEO_F767ZI,

    // L4
    NUCLEO_L412KB,
    NUCLEO_L412RB_P,
    NUCLEO_L432KC,
    NUCLEO_L433RC_P,
    NUCLEO_L452RE,
    NUCLEO_L452RE_P,
    NUCLEO_L476RG,
    NUCLEO_L496ZG,
    NUCLEO_L496ZG_P,
    NUCLEO_L4A6ZG,
    NUCLEO_L4R5ZI,
    NUCLEO_L4R5ZI_P,
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

macro_rules! board {
    ($form:ident, $family:ident, $flash_size:tt, $ram_size:tt) => {
        {
            let map = MemoryMap {
                flash: (0x08000000, $flash_size.to_string()),
                ram: (0x20000000, $ram_size.to_string()),
            };
            (Form::$form, Family::$family, map)
        }
    };
}

impl Feature {
    /// Return information about a specific board (feature)
    fn expand(&self) -> (Form, Family, MemoryMap) {
        match self {
            // F0
            Feature::NUCLEO_F030R8 => board!(Nucleo64,  Stm32F0xx, "256K", "32K"),
            Feature::NUCLEO_F042K6 => board!(Nucleo32,  Stm32F0xx, "32K",  "6K"),
            Feature::NUCLEO_F070RB => board!(Nucleo64,  Stm32F0xx, "128K", "16K"),

            // F4
            Feature::NUCLEO_F401RE => board!(Nucleo64,  Stm32F4xx, "64K",  "256K"),
            Feature::NUCLEO_F411RE => board!(Nucleo64,  Stm32F4xx, "512K", "128K"),
            Feature::NUCLEO_F412RG => board!(Nucleo64,  Stm32F4xx, "1M",   "256K"),
            Feature::NUCLEO_F412ZG => board!(Nucleo144, Stm32F4xx, "1M",   "256K"),
            Feature::NUCLEO_F429ZI => board!(Nucleo144, Stm32F4xx, "2M",   "256K"),

            // F7
            Feature::NUCLEO_F722ZE => board!(Nucleo144, Stm32F7xx, "512K", "256K"),
            Feature::NUCLEO_F746ZG => board!(Nucleo144, Stm32F7xx, "1M",   "320K"),
            Feature::NUCLEO_F756ZG => board!(Nucleo144, Stm32F7xx, "1M",   "320K"),
            Feature::NUCLEO_F767ZI => board!(Nucleo144, Stm32F7xx, "2M",   "512K"),

            // L4
            Feature::NUCLEO_L412KB   => board!(Nucleo32,  Stm32L4xx, "128K", "40K"),
            Feature::NUCLEO_L412RB_P => board!(Nucleo64,  Stm32L4xx, "128K", "40K"),
            Feature::NUCLEO_L432KC   => board!(Nucleo32,  Stm32L4xx, "256K", "64K"),
            Feature::NUCLEO_L433RC_P => board!(Nucleo64,  Stm32L4xx, "256K", "64K"),
            Feature::NUCLEO_L452RE   => board!(Nucleo64,  Stm32L4xx, "512K", "160K"),
            Feature::NUCLEO_L452RE_P => board!(Nucleo64,  Stm32L4xx, "512K", "160K"),
            Feature::NUCLEO_L476RG   => board!(Nucleo64,  Stm32L4xx, "1M",   "128K"),
            Feature::NUCLEO_L496ZG   => board!(Nucleo64,  Stm32L4xx, "1M",   "320K"),
            Feature::NUCLEO_L496ZG_P => board!(Nucleo144, Stm32L4xx, "1M",   "320K"),
            Feature::NUCLEO_L4A6ZG   => board!(Nucleo144, Stm32L4xx, "1M",   "320K"),
            Feature::NUCLEO_L4R5ZI   => board!(Nucleo144, Stm32L4xx, "2M",   "640K"),
            Feature::NUCLEO_L4R5ZI_P => board!(Nucleo144, Stm32L4xx, "2M",   "640K"),

        }
    }
}

/// Possible board form factors for STM32 Nucleo
#[derive(Debug)]
enum Form {
    Nucleo32,
    Nucleo64,
    Nucleo144,
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
