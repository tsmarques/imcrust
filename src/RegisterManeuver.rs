use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

/// Command used to indicate maneuver can be executed in the
/// vehicle.
#[derive(Default)]
pub struct RegisterManeuver {
    /// IMC Header
    pub header: Header,

    /// IMC serialization ID of maneuver type.
    pub _mid: u16,
}

impl Message for RegisterManeuver {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = RegisterManeuver {
            header: Header::new(469),

            _mid: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = RegisterManeuver {
            header: hdr,

            _mid: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        469
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        469
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._mid = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._mid);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._mid = bfr.get_u16_le();
    }
}
