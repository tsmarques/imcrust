use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

/// Waypoint coordinate of a Follow Path maneuver.
#[derive(Default)]
pub struct PathPoint {
    /// IMC Header
    pub header: Header,

    /// The North offset of the North/East/Down coordinate of this
    /// point in relation to the path start point.
    pub _x: f32,

    /// The East offset of the North/East/Down coordinate of this
    /// point in relation to the path start point.
    pub _y: f32,

    /// The Down offset of the North/East/Down coordinate of this
    /// point in relation to the path start point.
    pub _z: f32,
}

impl Message for PathPoint {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PathPoint {
            header: Header::new(458),

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PathPoint {
            header: hdr,

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        458
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        458
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        12
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._x = bfr.get_f32_le();

        self._y = bfr.get_f32_le();

        self._z = bfr.get_f32_le();
    }
}
