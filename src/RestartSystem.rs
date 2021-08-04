use crate::Message::*;

use crate::Header::Header;

/// Request the destination system to restart itself.
#[derive(Default)]
pub struct RestartSystem {
    /// IMC Header
    pub header: Header,
}

impl RestartSystem {
    pub fn new() -> RestartSystem {
        let mut msg = RestartSystem {
            header: Header::new(9),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for RestartSystem {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        9
    }

    fn clear(&mut self) {
        self.header.clear();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {}
}
