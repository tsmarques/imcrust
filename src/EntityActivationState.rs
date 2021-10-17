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
use crate::Header::Header;
use crate::Message::*;

/// State.
#[allow(non_camel_case_types)]
pub enum StateEnum {
    /// Entity is Inactive.
    EAS_INACTIVE = 0,
    /// Entity is Active.
    EAS_ACTIVE = 1,
    /// Activation in Progress.
    EAS_ACT_IP = 2,
    /// Activation Completed.
    EAS_ACT_DONE = 3,
    /// Activation Failed.
    EAS_ACT_FAIL = 4,
    /// Deactivation In Progress.
    EAS_DEACT_IP = 5,
    /// Deactivation Completed.
    EAS_DEACT_DONE = 6,
    /// Deactivation Failed.
    EAS_DEACT_FAIL = 7,
}

/// State of entity activation/deactivation.
#[derive(Default)]
pub struct EntityActivationState {
    /// Message Header.
    pub _header: Header,
    /// State.
    pub _state: u8,
    /// Error.
    pub _error: String,
}

impl Message for EntityActivationState {
    fn new() -> EntityActivationState
    where
        Self: Sized,
    {
        let msg = EntityActivationState {
            _header: Header::new(14),
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
        14
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        14
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(14);
        self._state = Default::default();
        self._error = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        serialize_bytes!(bfr, self._error.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._state = bfr.get_u8();
        deserialize_string!(bfr, self._error);
        Ok(())
    }
}
