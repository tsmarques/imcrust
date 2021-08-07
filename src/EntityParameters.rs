use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

use crate::EntityParameter::EntityParameter;

/// List of entity parameters.
#[derive(Default)]
pub struct EntityParameters {
    /// IMC Header
    pub header: Header,

    /// Name of the entity.
    pub _name: String,

    /// List of parameters.
    pub _params: MessageList<EntityParameter>,
}

impl Message for EntityParameters {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = EntityParameters {
            header: hdr,

            _name: Default::default(),
            _params: vec![],
        };

        msg.get_header()._mgid = 802;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = EntityParameters {
            header: Header::new(802),

            _name: Default::default(),
            _params: vec![],
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        802
    }

    fn id(&self) -> u16 {
        802
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        for msg in self._params.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        for msg in self._params.iter() {
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
        serialize_bytes!(bfr, self._name.as_bytes());
        for msg in self._params.iter() {
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
