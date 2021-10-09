use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Accepted
    SMS_ACCEPTED = 0,
    // Rejected
    SMS_REJECTED = 1,
    // Interrupted
    SMS_INTERRUPTED = 2,
    // Completed
    SMS_COMPLETED = 3,
    // Idle
    SMS_IDLE = 4,
    // Transmitting
    SMS_TRANSMITTING = 5,
    // Receiving
    SMS_RECEIVING = 6,
}

impl StateEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            SMS_ACCEPTED => 0,
            SMS_REJECTED => 1,
            SMS_INTERRUPTED => 2,
            SMS_COMPLETED => 3,
            SMS_IDLE => 4,
            SMS_TRANSMITTING => 5,
            SMS_RECEIVING => 6,
        }
    }
}

#[derive(Default)]
pub struct SmsState {
    /// IMC Header
    pub header: Header,

    /// Sequence number.
    pub _seq: u32,

    /// Current state of an SMS transaction.
    pub _state: u8,

    pub _error: String,
}

impl Message for SmsState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = SmsState {
            header: Header::new(159),

            _seq: Default::default(),
            _state: Default::default(),
            _error: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = SmsState {
            header: hdr,

            _seq: Default::default(),
            _state: Default::default(),
            _error: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        159
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        159
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();

        self._state = Default::default();

        self._error = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._seq);
        bfr.put_u8(self._state);
        serialize_bytes!(bfr, self._error.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._seq = bfr.get_u32_le();

        self._state = bfr.get_u8();

        deserialize_string!(bfr, self._error);
    }
}
