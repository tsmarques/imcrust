//###########################################################################
// Copyright 2017 OceanScan - Marine Systems & Technology, Lda.             #
//###########################################################################
// Licensed under the Apache License, Version 2.0 (the "License");          #
// you may not use this file except in compliance with the License.         #
// You may obtain a copy of the License at                                  #
//                                                                          #
// http://www.apache.org/licenses/LICENSE-2.0                               #
//                                                                          #
// Unless required by applicable law or agreed to in writing, software      #
// distributed under the License is distributed on an "AS IS" BASIS,        #
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. #
// See the License for the specific language governing permissions and      #
// limitations under the License.                                           #
//###########################################################################
// Author: Ricardo Martins                                                  #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Author: Tiago Sá Marques <tmarques@oceanscan-mst.com>

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Announce::Announce;
use crate::Header::Header;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// This message is sent by the TREX task which gives further information to a TREX instance about connected IMC nodes
#[derive(Default)]
pub struct VehicleLinks {
    /// Message Header.
    pub _header: Header,
    /// Local Name.
    pub _localname: String,
    /// Active Links.
    pub _links: MessageList<Announce>,
}

impl Message for VehicleLinks {
    fn new() -> VehicleLinks
    where
        Self: Sized,
    {
        let msg = VehicleLinks {
            _header: Header::new(650),
            _localname: Default::default(),
            _links: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        650
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        650
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(650);
        self._localname = Default::default();
        self._links = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._localname.len() + 2;
        message_list_serialization_size!(dyn_size, self._links);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._localname.as_bytes());
        serialize_message_list!(bfr, self._links);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._localname);
        self._links = deserialize_message_list_as::<Announce>(bfr)?;
        Ok(())
    }
}
