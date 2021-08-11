use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Report of conductivity.
#[derive(Default)]
pub struct Conductivity {
    /// IMC Header
    pub header: Header,

    /// The value of the conductivity as measured by the sensor.
    pub _value: f32,
}

impl Message for Conductivity {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Conductivity {
            header: Header::new(269),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Conductivity {
            header: hdr,

            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        269
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        269
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._value = bfr.get_f32_le();
    }
}
