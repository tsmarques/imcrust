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

/// An "Alignment" is a maneuver specifying a movement of the vehicle to a
/// target waypoint intended to control activation of an IMU/INS in order
/// to start aligning navigation for more precise dead reckoning operation.
/// Mandatory parameters defined for a "Launch" are
/// timeout, speed and speed units.
#[derive(Default)]
pub struct Alignment {
    /// Message Header.
    pub _header: Header,
    /// Timeout.
    pub _timeout: u16,
    /// Latitude WGS-84.
    pub _lat: f64,
    /// Longitude WGS-84.
    pub _lon: f64,
    /// Speed.
    pub _speed: f32,
    /// Speed Units.
    pub _speed_units: u8,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Alignment {
    fn new() -> Alignment {
        let msg = Alignment {
            _header: Header::new(495),
            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _speed: Default::default(),
            _speed_units: 0_u8,
            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        495
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        495
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(495);
        self._timeout = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        23
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._timeout);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timeout = bfr.get_u16_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
