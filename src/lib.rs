#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod enums;
mod update;

pub mod info;
pub mod protocol;
pub mod reader;
pub use enums::*;
pub use update::*;
mod reader_impl;

pub use reader::PacketReader;

use alloc::string::String;
use serde::{Deserialize, Serialize};

pub type VolumePercent = f32;
pub type AppIdentifier = u32;
pub type DeviceIdentifier = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(specta::Type))]
pub struct ProcessInfo {
    pub id: AppIdentifier,
    pub name: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(specta::Type))]
pub struct AudioApplication {
    pub process: ProcessInfo,
    pub session_type: SessionType,
    pub direction: SessionDirection,
    pub volume: AudioVolume,
    pub device_id: DeviceIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(specta::Type))]
pub struct AudioDevice {
    pub id: DeviceIdentifier,
    pub name: String,
    pub friendly_name: String,
    pub direction: SessionDirection,
    pub is_default: bool,
    pub volume: AudioVolume,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "bindings", derive(specta::Type))]
pub struct AudioVolume {
    pub current: VolumePercent,
    pub muted: bool,
}

impl AudioVolume {
    pub fn new(volume: VolumePercent) -> Self {
        Self {
            current: volume,
            muted: false,
        }
    }
}
