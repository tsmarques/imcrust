use crate::Message::*;

use crate::Header::Header;

/// Command to obtain the operational limits in use by the vehicle.
#[derive(Default)]
pub struct GetOperationalLimits {
    /// IMC Header
    pub header: Header,
}

impl GetOperationalLimits {
    pub fn new() -> GetOperationalLimits {
        let mut msg = GetOperationalLimits {
            header: Header::new(505),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for GetOperationalLimits {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        505
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
