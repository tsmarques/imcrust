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
// Author: Tiago Sá Marques                                                 #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Type of measurement.
#[allow(non_camel_case_types)]
pub enum TypeofmeasurementEnum {
    /// Colored.
    DT_COLORED = 0,
    /// Fluorescent.
    DT_FLUORESCENT = 1,
}

/// Dissolved Organic Matter measurement.
#[derive(Default)]
pub struct DissolvedOrganicMatter {
    /// Message Header.
    pub _header: Header,
    /// Value.
    pub _value: f32,
    /// Type of measurement.
    pub _type: u8,
}

impl Message for DissolvedOrganicMatter {
    fn new() -> DissolvedOrganicMatter {
        let msg = DissolvedOrganicMatter {
            _header: Header::new(903),
            _value: Default::default(),
            _type: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        903
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        903
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(903);
        self._value = Default::default();
        self._type = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
        bfr.put_u8(self._type);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._value = bfr.get_f32_le();
        self._type = bfr.get_u8();
        Ok(())
    }
}
