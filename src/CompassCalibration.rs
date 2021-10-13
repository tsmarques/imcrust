use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum DirectionEnum {
    // Vehicle Dependent
    LD_VDEP = 0,
    // Clockwise
    LD_CLOCKW = 1,
    // Counter Clockwise
    LD_CCLOCKW = 2,
    // Into the wind/current
    LD_IWINDCURR = 3,
}

#[allow(non_camel_case_types)]
pub mod Direction {
    // Vehicle Dependent
    pub const LD_VDEP: u32 = 0;
    // Clockwise
    pub const LD_CLOCKW: u32 = 1;
    // Counter Clockwise
    pub const LD_CCLOCKW: u32 = 2;
    // Into the wind/current
    pub const LD_IWINDCURR: u32 = 3;
}

/// message-group: Maneuver
// impl Maneuver for CompassCalibration { }

/// This maneuver is a mix between the Loiter maneuver and the YoYo maneuver.
/// The vehicle cirlcles around a specific waypoint with a variable Z
/// reference between a minimum and maximum value.
/// message-group: Maneuver
#[derive(Default)]
pub struct CompassCalibration {
    /// IMC Header
    pub header: Header,

    /// The timeout indicates the time that an error should occur if
    /// exceeded.
    pub _timeout: u16,

    /// WGS-84 Latitude coordinate.
    pub _lat: f64,

    /// WGS-84 Longitude coordinate.
    pub _lon: f64,

    /// Maneuver reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,

    /// Units of the z reference.
    pub _z_units: u8,

    /// Pitch angle used to perform the maneuver.
    pub _pitch: f32,

    /// Yoyo motion amplitude.
    pub _amplitude: f32,

    /// The duration of this maneuver. Use '0' for unlimited duration time.
    pub _duration: u16,

    /// Maneuver speed.
    pub _speed: f32,

    /// Speed units.
    pub _speed_units: u8,

    /// Radius of the maneuver.
    pub _radius: f32,

    /// Direction of the maneuver.
    pub _direction: u8,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for CompassCalibration {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = CompassCalibration {
            header: Header::new(475),

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _pitch: Default::default(),
            _amplitude: Default::default(),
            _duration: Default::default(),
            _speed: Default::default(),
            _speed_units: 0_u8,
            _radius: Default::default(),
            _direction: Default::default(),
            _custom: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = CompassCalibration {
            header: hdr,

            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _pitch: Default::default(),
            _amplitude: Default::default(),
            _duration: Default::default(),
            _speed: Default::default(),
            _speed_units: 0_u8,
            _radius: Default::default(),
            _direction: Default::default(),
            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        475
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        475
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
        self._pitch = Default::default();
        self._amplitude = Default::default();
        self._duration = Default::default();
        self._speed = Default::default();
        self._speed_units = Default::default();
        self._radius = Default::default();
        self._direction = Default::default();
        self._custom = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        43
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
        bfr.put_f32_le(self._pitch);
        bfr.put_f32_le(self._amplitude);
        bfr.put_u16_le(self._duration);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_f32_le(self._radius);
        bfr.put_u8(self._direction);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timeout = bfr.get_u16_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        self._pitch = bfr.get_f32_le();
        self._amplitude = bfr.get_f32_le();
        self._duration = bfr.get_u16_le();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._radius = bfr.get_f32_le();
        self._direction = bfr.get_u8();
        deserialize_string!(bfr, self._custom);

        Ok(())
    }
}
