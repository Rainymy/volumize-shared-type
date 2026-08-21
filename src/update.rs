use alloc::string::String;
use serde::{Deserialize, Serialize};

use super::{AppIdentifier, DeviceIdentifier};

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
