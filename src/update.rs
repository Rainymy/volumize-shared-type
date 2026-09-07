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

impl UpdateChange {
    pub fn volume_change(id: Identifier, volume: f32, mute: bool) -> Self {
        Self {
            id: id,
            change: ChangeType::AudioVolume { volume, mute },
        }
    }

    pub fn app_icon_change(id: Identifier, path: String) -> Self {
        Self {
            id: id,
            change: ChangeType::IconPathChange { path },
        }
    }

    pub fn app_name_change(id: Identifier, name: String) -> Self {
        Self {
            id: id,
            change: ChangeType::NameChange { name },
        }
    }

    pub fn app_state_change(id: Identifier, state: EntityState) -> Self {
        Self {
            id: id,
            change: ChangeType::StateChange { state },
        }
    }
}
