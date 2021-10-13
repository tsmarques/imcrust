use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum TypeEnum {
    // X-axis
    CD_X = 0x01,
    // Y-axis
    CD_Y = 0x02,
    // Z-axis
    CD_Z = 0x04,
    // Impact
    CD_IMPACT = 0x08,
}

#[allow(non_camel_case_types)]
pub mod Type {
    // X-axis
    pub const CD_X: u32 = 0x01;
    // Y-axis
    pub const CD_Y: u32 = 0x02;
    // Z-axis
    pub const CD_Z: u32 = 0x04;
    // Impact
    pub const CD_IMPACT: u32 = 0x08;
}

/// Collision detected in the z-axis
#[derive(Default)]
pub struct Collision {
    /// IMC Header
    pub header: Header,

    /// Estimated collision acceleration value.
    pub _value: f32,

    /// Sudden impact detected
    pub _type: u8,
}

impl Message for Collision {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Collision {
            header: Header::new(509),

            _value: Default::default(),
            _type: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Collision {
            header: hdr,

            _value: Default::default(),
            _type: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        509
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        509
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
        self._type = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
        bfr.put_u8(self._type);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._value = bfr.get_f32_le();
        self._type = bfr.get_u8();

        Ok(())
    }
}
