use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrameError<E> {
    Io(E),
    OversizedFrame { len: usize, max: usize },
}

impl<E> From<E> for FrameError<E> {
    fn from(e: E) -> Self {
        FrameError::Io(e)
    }
}

pub trait PacketReader {
    type Error;

    fn max_packet_size(&self) -> u16;
    fn read_packet(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize, Self::Error>>;
}

#[cfg(feature = "usb")]
mod usb_impl {
    use super::PacketReader;
    use embassy_usb::{
        class::cdc_acm::Receiver,
        driver::{Driver, EndpointError},
    };

    impl<'a, D: Driver<'a>> PacketReader for Receiver<'a, D> {
        type Error = EndpointError;

        fn max_packet_size(&self) -> u16 {
            Receiver::max_packet_size(self)
        }

        async fn read_packet(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
            Receiver::read_packet(self, buf).await
        }
    }
}

#[cfg(feature = "std")]
mod std_impl {
    use super::PacketReader;
    use std::io::Read;

    impl<R: Read> PacketReader for R {
        type Error = std::io::Error;

        fn max_packet_size(&self) -> u16 {
            1024
        }

        async fn read_packet(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            // Unsure if this is the correct way to read within async context
            // TODO: Read into the buffer asynchronously
            self.read(buf)
        }
    }
}

#[cfg(feature = "tokio")]
mod tokio_impl {
    use super::PacketReader;
    use tokio::io::AsyncRead;

    impl<R: AsyncRead> PacketReader for R {
        type Error = tokio::io::Error;

        fn max_packet_size(&self) -> u16 {
            1024
        }

        async fn read_packet(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            self.read(buf).await
        }
    }
}

use alloc::fmt;
impl<E: fmt::Display> fmt::Display for FrameError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameError::Io(e) => write!(f, "transport error: {e}"),
            FrameError::OversizedFrame { len, max } => {
                write!(f, "frame too large: {len} bytes (max {max})")
            }
        }
    }
}

pub async fn read_frame<R: PacketReader>(reader: &mut R) -> Result<Vec<u8>, FrameError<R::Error>> {
    const MAX_FRAME_LEN: usize = 1024;
    const HEADER_LEN: usize = 2;

    let mut packet_buf = vec![0u8; reader.max_packet_size() as usize];
    let mut header = [0u8; HEADER_LEN];
    let mut header_len = 0;
    let mut payload = Vec::new();

    // Extract length (2 bytes) from the packet
    while header_len < HEADER_LEN {
        let n = reader.read_packet(&mut packet_buf).await?;
        let bytes = &packet_buf[..n];

        let need = HEADER_LEN - header_len;
        let take = need.min(bytes.len());
        header[header_len..header_len + take].copy_from_slice(&bytes[..take]);
        header_len += take;

        if take < bytes.len() {
            payload.extend_from_slice(&bytes[take..]);
        }
    }

    let frame_len = u16::from_le_bytes(header) as usize;
    if frame_len > MAX_FRAME_LEN {
        return Err(FrameError::OversizedFrame {
            len: frame_len,
            max: MAX_FRAME_LEN,
        });
    }

    // In case the leftover header-packet bytes overshot the frame.
    payload.truncate(frame_len);

    // Keep reading packets until we have the full payload.
    while payload.len() < frame_len {
        let n = reader.read_packet(&mut packet_buf).await?;
        let remaining = frame_len - payload.len();
        // In case it reads more than we need, take only the remaining bytes
        let take = n.min(remaining);
        payload.extend_from_slice(&packet_buf[..take]);
    }

    Ok(payload)
}
