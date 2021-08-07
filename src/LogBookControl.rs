use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

use crate::LogBookEntry::LogBookEntry;

#[allow(non_camel_case_types)]
pub enum CommandEnum {
    // Get
    LBC_GET = 0,
    // Clear
    LBC_CLEAR = 1,
    // Get Errors
    LBC_GET_ERR = 2,
    // Reply
    LBC_REPLY = 3,
}

impl CommandEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            LBC_GET => 0,
            LBC_CLEAR => 1,
            LBC_GET_ERR => 2,
            LBC_REPLY => 3,
        }
    }
}

/// Retrieve log book entries corresponding to errors.
#[derive(Default)]
pub struct LogBookControl {
    /// IMC Header
    pub header: Header,

    /// Reply to a GET command. Message argument is a MessageList
    /// instance containing LogBookEntry messages.
    pub _command: u8,

    /// Timestamp for command (Epoch time).
    pub _htime: f64,

    /// Argument, currently used only for 'REPLY'.
    pub _msg: MessageList<LogBookEntry>,
}

impl Message for LogBookControl {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = LogBookControl {
            header: hdr,

            _command: Default::default(),
            _htime: Default::default(),
            _msg: vec![],
        };

        msg.get_header()._mgid = 104;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = LogBookControl {
            header: Header::new(104),

            _command: Default::default(),
            _htime: Default::default(),
            _msg: vec![],
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        104
    }

    fn id(&self) -> u16 {
        104
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._command = Default::default();

        self._htime = Default::default();

        for msg in self._msg.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        9
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        for msg in self._msg.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
            }
        }

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._command);
        bfr.put_f64_le(self._htime);
        for msg in self._msg.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
