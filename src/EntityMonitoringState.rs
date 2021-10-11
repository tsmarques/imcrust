use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

#[derive(Default)]
pub struct EntityMonitoringState {
    /// IMC Header
    pub header: Header,

    /// Number of entitities being monitored.
    pub _mcount: u8,

    /// Comma separated list of all entity names being monitored.
    pub _mnames: String,

    /// Number of entitities with non-critical errors.
    pub _ecount: u8,

    /// Comma separated list of all entity names with non-critical
    /// errors.
    pub _enames: String,

    /// Number of entitities with critical errors.
    pub _ccount: u8,

    /// Comma separated list of all entity names with critical errors.
    pub _cnames: String,

    /// Description of last error.
    pub _last_error: String,

    /// Time of last error (Epoch time).
    pub _last_error_time: f64,
}

impl Message for EntityMonitoringState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = EntityMonitoringState {
            header: Header::new(503),

            _mcount: Default::default(),
            _mnames: Default::default(),
            _ecount: Default::default(),
            _enames: Default::default(),
            _ccount: Default::default(),
            _cnames: Default::default(),
            _last_error: Default::default(),
            _last_error_time: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = EntityMonitoringState {
            header: hdr,

            _mcount: Default::default(),
            _mnames: Default::default(),
            _ecount: Default::default(),
            _enames: Default::default(),
            _ccount: Default::default(),
            _cnames: Default::default(),
            _last_error: Default::default(),
            _last_error_time: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        503
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        503
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._mcount = Default::default();

        self._mnames = Default::default();

        self._ecount = Default::default();

        self._enames = Default::default();

        self._ccount = Default::default();

        self._cnames = Default::default();

        self._last_error = Default::default();

        self._last_error_time = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        11
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._mnames.len() + 2;

        dyn_size += self._enames.len() + 2;

        dyn_size += self._cnames.len() + 2;

        dyn_size += self._last_error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._mcount);
        serialize_bytes!(bfr, self._mnames.as_bytes());
        bfr.put_u8(self._ecount);
        serialize_bytes!(bfr, self._enames.as_bytes());
        bfr.put_u8(self._ccount);
        serialize_bytes!(bfr, self._cnames.as_bytes());
        serialize_bytes!(bfr, self._last_error.as_bytes());
        bfr.put_f64_le(self._last_error_time);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._mcount = bfr.get_u8();

        deserialize_string!(bfr, self._mnames);

        self._ecount = bfr.get_u8();

        deserialize_string!(bfr, self._enames);

        self._ccount = bfr.get_u8();

        deserialize_string!(bfr, self._cnames);

        deserialize_string!(bfr, self._last_error);

        self._last_error_time = bfr.get_f64_le();
    }
}
