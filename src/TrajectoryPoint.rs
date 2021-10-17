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

/// Waypoint coordinate of a Follow Trajectory maneuver.
#[derive(Default)]
pub struct TrajectoryPoint {
    /// Message Header.
    pub _header: Header,
    /// North Offset (m).
    pub _x: f32,
    /// East Offset (m).
    pub _y: f32,
    /// Down Offset (m).
    pub _z: f32,
    /// Time Offset (s).
    pub _t: f32,
}

impl Message for TrajectoryPoint {
    fn new() -> TrajectoryPoint
    where
        Self: Sized,
    {
        let msg = TrajectoryPoint {
            _header: Header::new(464),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _t: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        464
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        464
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(464);
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._t = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        16
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._t);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        self._t = bfr.get_f32_le();
        Ok(())
    }
}
