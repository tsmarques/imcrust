use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Location of a specific device in the system infrastructure.
#[derive(Default)]
pub struct DeviceState {
    /// IMC Header
    pub header: Header,

    /// Device's position over the X axis.
    pub _x: f32,

    /// Device's position over the Y axis.
    pub _y: f32,

    /// Device's position over the Z axis.
    pub _z: f32,

    /// Device's rotation over the X axis.
    pub _phi: f32,

    /// Device's rotation over the Y axis.
    pub _theta: f32,

    /// Device's rotation over the Z axis.
    pub _psi: f32,
}

impl Message for DeviceState {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = DeviceState {
            header: hdr,

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
        };

        msg.get_header()._mgid = 282;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = DeviceState {
            header: Header::new(282),

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        282
    }

    fn id(&self) -> u16 {
        282
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();

        self._phi = Default::default();

        self._theta = Default::default();

        self._psi = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        24
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._phi);
        bfr.put_f32_le(self._theta);
        bfr.put_f32_le(self._psi);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
