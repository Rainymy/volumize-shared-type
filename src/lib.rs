#![no_std]
extern crate alloc;

use alloc::string::String;
use serde::{Deserialize, Serialize};

pub type VolumePercent = f32;
pub type AppIdentifier = u32;
pub type DeviceIdentifier = String;
pub const UPDATE_EVENT_NAME: &str = "update";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum Identifier {
    App(AppIdentifier),
    Device(DeviceIdentifier),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ChangeType {
    AudioVolume { volume: f32, mute: bool },
    IconPathChange { path: String },
    StateChange { state: EntityState },
    NameChange { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityState {
    Disconnect,
    Created,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChange {
    pub id: Identifier,
    pub change: ChangeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    Application,
    Device,
    System,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionDirection {
    Render,
    Capture,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub id: AppIdentifier,
    pub name: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioApplication {
    pub process: ProcessInfo,
    pub session_type: SessionType,
    pub direction: SessionDirection,
    pub volume: AudioVolume,
    pub device_id: DeviceIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: DeviceIdentifier,
    pub name: String,
    pub friendly_name: String,
    pub direction: SessionDirection,
    pub is_default: bool,
    pub volume: AudioVolume,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
