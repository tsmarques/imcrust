use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Chlorophyll measurement.
#[derive(Default)]
pub struct Chlorophyll {
    /// IMC Header
    pub header: Header,

    /// Chlorophyll reading.
    pub _value: f32,
}

impl Chlorophyll {
    pub fn new() -> Chlorophyll {
        let mut msg = Chlorophyll {
            header: Header::new(289),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Chlorophyll {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        289
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
    }
}
