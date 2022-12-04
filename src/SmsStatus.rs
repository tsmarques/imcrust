//###########################################################################
// Copyright 2021 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// Status
#[allow(non_camel_case_types)]
pub enum StatusEnum {
    /// Queued
    SMSSTAT_QUEUED = 0,
    /// Sent
    SMSSTAT_SENT = 1,
    /// Input Error
    SMSSTAT_INPUT_FAILURE = 101,
    /// Error trying to send sms
    SMSSTAT_ERROR = 102,
}

/// Reply sent in response to a SMS sending request.
#[derive(Default, Clone)]
pub struct SmsStatus {
    /// Message Header
    pub _header: Header,
    pub _req_id: u16,
    pub _status: u8,
    /// Error description.
    pub _info: String,
}

impl Message for SmsStatus {
    fn new() -> SmsStatus {
        

        SmsStatus {
            _header: Header::new(518),
            _req_id: Default::default(),
            _status: Default::default(),
            _info: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        518
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        518
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(518);
        self._req_id = Default::default();
        self._status = Default::default();
        self._info = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        bfr.put_u8(self._status);
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._req_id = bfr.get_u16_le();
        self._status = bfr.get_u8();
        deserialize_string!(bfr, self._info);
        Ok(())
    }
}
