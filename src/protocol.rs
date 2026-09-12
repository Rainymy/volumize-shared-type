use core::sync::atomic::{AtomicU32, Ordering};

use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use super::*;

pub type RequestId = u32;

static NEXT_REQUEST_ID: AtomicU32 = AtomicU32::new(1);
pub fn next_request_id() -> RequestId {
    NEXT_REQUEST_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(specta::Type))]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Command {
    GetVolume {
        id: Identifier,
    },
    SetVolume {
        id: Identifier,
        volume: VolumePercent,
    },
    SetMute {
        id: Identifier,
        mute: bool,
    },
    GetApplications {
        id: DeviceIdentifier,
    },
    GetApplication {
        id: AppIdentifier,
    },
    GetIcon {
        id: Identifier,
    },
    GetPlaybackDevices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(specta::Type))]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Response {
    Volume {
        id: Identifier,
        volume: AudioVolume,
    },
    ApplicationList {
        id: DeviceIdentifier,
        apps: Vec<AppIdentifier>,
    },
    Application(AudioApplication),
    Icon {
        id: AppIdentifier,
        data: Vec<u8>,
    },
    DeviceList(Vec<AudioDevice>),
    Error {
        message: String,
    },
    ACK,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(specta::Type))]
pub struct CommandRequest {
    pub id: RequestId,
    pub command: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bindings", derive(specta::Type))]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct CommandResponse {
    pub id: RequestId,
    pub response: Response,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum Envelope {
    Command(CommandRequest),
    Response(CommandResponse),
    Event(UpdateChange),
}

pub struct RawFrame(Vec<u8>);

impl RawFrame {
    pub fn encode(envelope: &Envelope) -> RawFrame {
        let mut buffer = Vec::new();
        // Return type is Infallible. So unwraping should be safe.
        let _result = ciborium::into_writer(&envelope, &mut buffer);
        Self(buffer)
    }

    pub fn decode(buffer: &[u8]) -> Result<Envelope, String> {
        use alloc::string::ToString;
        ciborium::from_reader(buffer).map_err(|e| e.to_string())
    }

    pub fn build(self) -> Vec<u8> {
        let buffer_len = self.0.len().to_le_bytes();
        let mut buffer = Vec::with_capacity(buffer_len.len() + self.0.len());

        buffer.extend_from_slice(&buffer_len);
        buffer.extend_from_slice(&self.0);
        buffer
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
