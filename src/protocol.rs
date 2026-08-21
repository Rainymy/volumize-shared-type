use alloc::vec::Vec;
use embassy_usb::{
    class::cdc_acm::Receiver,
    driver::{Driver, EndpointError},
};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        device_id: DeviceIdentifier,
    },
    GetApplication {
        app_id: AppIdentifier,
    },
    GetIcon {
        app_id: AppIdentifier,
    },
    GetPlaybackDevices,
}

use alloc::string::String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Volume {
        id: Identifier,
        volume: AudioVolume,
    },
    ApplicationList {
        device_id: DeviceIdentifier,
        apps: Vec<AppIdentifier>,
    },
    Application(AudioApplication),
    Icon {
        app_id: AppIdentifier,
        data: Vec<u8>,
    },
    DeviceList(Vec<AudioDevice>),
    Error {
        request: Command,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum Envelope {
    Command(Command),
    Response(Response),
    Event(UpdateChange),
}

pub async fn read_frame<'a>(
    class: &mut Receiver<'a, impl Driver<'a>>,
) -> Result<Vec<u8>, EndpointError> {
    use alloc::vec;

    const MAX_FRAME_LEN: usize = 1024;
    let mut packet_buf = vec![0u8; class.max_packet_size() as usize];

    let mut header_buf = [0u8; 2];
    let mut len_have = 0;

    // Extract length (2 bytes) from the packet
    while len_have < header_buf.len() {
        let n = class.read_packet(&mut packet_buf).await?;
        for &b in &packet_buf[..n] {
            if len_have < header_buf.len() {
                header_buf[len_have] = b;
                len_have += 1;
            }
            // What if the host sends more than 2 bytes?
            // For now, we just ignore the extra bytes.
        }
    }

    let frame_len = u16::from_le_bytes(header_buf) as usize;
    if frame_len > MAX_FRAME_LEN {
        return Err(EndpointError::BufferOverflow); // reject oversized/garbage frame
    }

    // Keep reading packets until we have accumulated the full payload
    let mut payload = Vec::new();
    while payload.len() < frame_len {
        let n = class.read_packet(&mut packet_buf).await?;
        let remaining = frame_len - payload.len();
        // In case it reads more than we need, take only the remaining bytes
        let take = n.min(remaining);
        payload.extend_from_slice(&packet_buf[..take]);
    }

    Ok(payload)
}

pub struct RawFrame(Vec<u8>);

impl RawFrame {
    pub fn build(self) -> Vec<u8> {
        let buffer_len = self.0.len().to_le_bytes();
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&buffer_len);
        buffer.extend_from_slice(&self.0);

        buffer
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub fn encode_frame(envelope: &Envelope) -> Result<RawFrame, String> {
    use alloc::string::ToString;

    let mut buffer = Vec::new();
    ciborium::into_writer(&envelope, &mut buffer).map_err(|e| e.to_string())?;

    Ok(RawFrame(buffer))
}
