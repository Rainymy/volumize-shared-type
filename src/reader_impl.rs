#[cfg(feature = "usb")]
mod usb_impl {
    use super::super::PacketReader;

    use embassy_usb::{
        class::cdc_acm::Receiver,
        driver::{Driver, EndpointError},
    };

    impl<'a, D: Driver<'a>> PacketReader for Receiver<'a, D> {
        type Error = EndpointError;

        fn max_packet_size(&self) -> u16 {
            Receiver::max_packet_size(self)
        }

        async fn read_packet(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            Receiver::read_packet(self, buf).await
        }
    }
}

#[cfg(feature = "std")]
mod std_impl {
    use super::super::PacketReader;
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
    use super::super::PacketReader;

    use core::marker::Unpin;
    use tokio::io::AsyncReadExt;

    impl<R: AsyncReadExt + Unpin> PacketReader for R {
        type Error = tokio::io::Error;

        fn max_packet_size(&self) -> u16 {
            1024
        }

        async fn read_packet(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            self.read(buf).await
        }
    }
}
