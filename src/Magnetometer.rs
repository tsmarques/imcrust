use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

#[allow(non_camel_case_types)]
pub enum DirectionEnum {
    // Clockwise First
    MD_CLOCKW_FIRST = 0,
    // Counter Clockwise First
    MD_CCLOCKW_FIRST = 1,
}

impl DirectionEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            MD_CLOCKW_FIRST => 0,
            MD_CCLOCKW_FIRST => 1,
        }
    }
}

/// message-group: Maneuver
impl Maneuver for Magnetometer {}

/// Magnetometer calibration maneuver (i.e: one square pattern
/// in one direction, a second square in the opposite direction)
/// message-group: Maneuver
#[derive(Default)]
pub struct Magnetometer {
    /// IMC Header
    pub header: Header,

    /// The amount of time the maneuver is allowed to run.
    pub _timeout: u16,

    /// WGS-84 Latitude of target waypoint.
    pub _lat: f64,

    /// WGS-84 Longitude of target waypoint.
    pub _lon: f64,

    /// Maneuver reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,

    /// Units of the z reference.
    pub _z_units: u8,

    /// Maneuver speed reference.
    pub _speed: f32,

    /// Speed units.
    pub _speed_units: u8,

    /// Rows bearing angle.
    pub _bearing: f64,

    /// Width of the maneuver.
    pub _width: f32,

    /// Desired direction.
    pub _direction: u8,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Magnetometer {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = Magnetometer {
            header: hdr,

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _bearing: Default::default(),
            _width: Default::default(),
            _direction: Default::default(),
            _custom: Default::default(),
        };

        msg.get_header()._mgid = 499;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = Magnetometer {
            header: Header::new(499),

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _bearing: Default::default(),
            _width: Default::default(),
            _direction: Default::default(),
            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        499
    }

    fn id(&self) -> u16 {
        499
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timeout = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._z = Default::default();

        self._z_units = Default::default();

        self._speed = Default::default();

        self._speed_units = Default::default();

        self._bearing = Default::default();

        self._width = Default::default();

        self._direction = Default::default();

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        41
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._timeout);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_f64_le(self._bearing);
        bfr.put_f32_le(self._width);
        bfr.put_u8(self._direction);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
