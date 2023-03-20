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
// IMC XML MD5: b521199aa61f91939b6b6ed9e44d149b                            *
//###########################################################################

use crate::Header;
use crate::Message;
/// Base
use crate::*;

pub fn buildFrom<T: Message>(hdr: Header) -> Option<T> {
    let mut msg: T = T::new();
    if msg.id() != hdr._mgid {
        return None;
    }
    msg.get_mut_header()._mgid = hdr._mgid;
    msg.get_mut_header()._sync = hdr._sync;
    msg.get_mut_header()._size = hdr._size;
    msg.get_mut_header()._timestamp = hdr._timestamp;
    msg.get_mut_header()._src = hdr._src;
    msg.get_mut_header()._src_ent = hdr._src_ent;
    msg.get_mut_header()._dst = hdr._dst;
    msg.get_mut_header()._dst_ent = hdr._dst_ent;
    Some(msg)
}

pub fn build(hdr: Header) -> Option<Box<dyn Message>> {
    let ret = buildFromId(hdr._mgid);
    if ret.is_none() {
        return ret;
    }

    let mut msg = ret.unwrap();
    msg.get_mut_header()._dst = hdr._dst;
    msg.get_mut_header()._size = hdr._size;
    msg.get_mut_header()._sync = hdr._sync;
    msg.get_mut_header()._src = hdr._src;
    msg.get_mut_header()._dst_ent = hdr._dst_ent;
    msg.get_mut_header()._src_ent = hdr._src_ent;
    msg.get_mut_header()._timestamp = hdr._timestamp;

    Option::from(msg)
}

pub fn buildFromId(id: u16) -> Option<Box<dyn Message>> {
    match id {
        1 => Some(Box::new(EntityState::new())),
        2 => Some(Box::new(QueryEntityState::new())),
        3 => Some(Box::new(EntityInfo::new())),
        4 => Some(Box::new(QueryEntityInfo::new())),
        5 => Some(Box::new(EntityList::new())),
        7 => Some(Box::new(CpuUsage::new())),
        8 => Some(Box::new(TransportBindings::new())),
        9 => Some(Box::new(RestartSystem::new())),
        12 => Some(Box::new(DevCalibrationControl::new())),
        13 => Some(Box::new(DevCalibrationState::new())),
        14 => Some(Box::new(EntityActivationState::new())),
        15 => Some(Box::new(QueryEntityActivationState::new())),
        16 => Some(Box::new(VehicleOperationalLimits::new())),
        20 => Some(Box::new(MsgList::new())),
        50 => Some(Box::new(SimulatedState::new())),
        51 => Some(Box::new(LeakSimulation::new())),
        52 => Some(Box::new(UASimulation::new())),
        53 => Some(Box::new(DynamicsSimParam::new())),
        100 => Some(Box::new(StorageUsage::new())),
        101 => Some(Box::new(CacheControl::new())),
        102 => Some(Box::new(LoggingControl::new())),
        103 => Some(Box::new(LogBookEntry::new())),
        104 => Some(Box::new(LogBookControl::new())),
        105 => Some(Box::new(ReplayControl::new())),
        106 => Some(Box::new(ClockControl::new())),
        107 => Some(Box::new(HistoricCTD::new())),
        108 => Some(Box::new(HistoricTelemetry::new())),
        109 => Some(Box::new(HistoricSonarData::new())),
        110 => Some(Box::new(HistoricEvent::new())),
        111 => Some(Box::new(VerticalProfile::new())),
        112 => Some(Box::new(ProfileSample::new())),
        150 => Some(Box::new(Heartbeat::new())),
        151 => Some(Box::new(Announce::new())),
        152 => Some(Box::new(AnnounceService::new())),
        153 => Some(Box::new(RSSI::new())),
        154 => Some(Box::new(VSWR::new())),
        155 => Some(Box::new(LinkLevel::new())),
        156 => Some(Box::new(Sms::new())),
        157 => Some(Box::new(SmsTx::new())),
        158 => Some(Box::new(SmsRx::new())),
        159 => Some(Box::new(SmsState::new())),
        160 => Some(Box::new(TextMessage::new())),
        170 => Some(Box::new(IridiumMsgRx::new())),
        171 => Some(Box::new(IridiumMsgTx::new())),
        172 => Some(Box::new(IridiumTxStatus::new())),
        180 => Some(Box::new(GroupMembershipState::new())),
        181 => Some(Box::new(SystemGroup::new())),
        182 => Some(Box::new(LinkLatency::new())),
        183 => Some(Box::new(ExtendedRSSI::new())),
        184 => Some(Box::new(HistoricData::new())),
        185 => Some(Box::new(CompressedHistory::new())),
        186 => Some(Box::new(HistoricSample::new())),
        187 => Some(Box::new(HistoricDataQuery::new())),
        188 => Some(Box::new(RemoteCommand::new())),
        189 => Some(Box::new(CommSystemsQuery::new())),
        190 => Some(Box::new(TelemetryMsg::new())),
        200 => Some(Box::new(LblRange::new())),
        202 => Some(Box::new(LblBeacon::new())),
        203 => Some(Box::new(LblConfig::new())),
        206 => Some(Box::new(AcousticMessage::new())),
        207 => Some(Box::new(SimAcousticMessage::new())),
        211 => Some(Box::new(AcousticOperation::new())),
        212 => Some(Box::new(AcousticSystemsQuery::new())),
        213 => Some(Box::new(AcousticSystems::new())),
        214 => Some(Box::new(AcousticLink::new())),
        215 => Some(Box::new(AcousticRequest::new())),
        216 => Some(Box::new(AcousticStatus::new())),
        217 => Some(Box::new(AcousticRelease::new())),
        250 => Some(Box::new(Rpm::new())),
        251 => Some(Box::new(Voltage::new())),
        252 => Some(Box::new(Current::new())),
        253 => Some(Box::new(GpsFix::new())),
        254 => Some(Box::new(EulerAngles::new())),
        255 => Some(Box::new(EulerAnglesDelta::new())),
        256 => Some(Box::new(AngularVelocity::new())),
        257 => Some(Box::new(Acceleration::new())),
        258 => Some(Box::new(MagneticField::new())),
        259 => Some(Box::new(GroundVelocity::new())),
        260 => Some(Box::new(WaterVelocity::new())),
        261 => Some(Box::new(VelocityDelta::new())),
        262 => Some(Box::new(Distance::new())),
        263 => Some(Box::new(Temperature::new())),
        264 => Some(Box::new(Pressure::new())),
        265 => Some(Box::new(Depth::new())),
        266 => Some(Box::new(DepthOffset::new())),
        267 => Some(Box::new(SoundSpeed::new())),
        268 => Some(Box::new(WaterDensity::new())),
        269 => Some(Box::new(Conductivity::new())),
        270 => Some(Box::new(Salinity::new())),
        271 => Some(Box::new(WindSpeed::new())),
        272 => Some(Box::new(RelativeHumidity::new())),
        273 => Some(Box::new(DevDataText::new())),
        274 => Some(Box::new(DevDataBinary::new())),
        275 => Some(Box::new(Force::new())),
        276 => Some(Box::new(SonarData::new())),
        277 => Some(Box::new(Pulse::new())),
        278 => Some(Box::new(PulseDetectionControl::new())),
        279 => Some(Box::new(FuelLevel::new())),
        280 => Some(Box::new(GpsNavData::new())),
        281 => Some(Box::new(ServoPosition::new())),
        282 => Some(Box::new(DeviceState::new())),
        283 => Some(Box::new(BeamConfig::new())),
        284 => Some(Box::new(DataSanity::new())),
        285 => Some(Box::new(RhodamineDye::new())),
        286 => Some(Box::new(CrudeOil::new())),
        287 => Some(Box::new(FineOil::new())),
        288 => Some(Box::new(Turbidity::new())),
        289 => Some(Box::new(Chlorophyll::new())),
        290 => Some(Box::new(Fluorescein::new())),
        291 => Some(Box::new(Phycocyanin::new())),
        292 => Some(Box::new(Phycoerythrin::new())),
        293 => Some(Box::new(GpsFixRtk::new())),
        294 => Some(Box::new(ExternalNavData::new())),
        295 => Some(Box::new(DissolvedOxygen::new())),
        296 => Some(Box::new(AirSaturation::new())),
        297 => Some(Box::new(Throttle::new())),
        298 => Some(Box::new(PH::new())),
        299 => Some(Box::new(Redox::new())),
        300 => Some(Box::new(CameraZoom::new())),
        301 => Some(Box::new(SetThrusterActuation::new())),
        302 => Some(Box::new(SetServoPosition::new())),
        303 => Some(Box::new(SetControlSurfaceDeflection::new())),
        304 => Some(Box::new(RemoteActionsRequest::new())),
        305 => Some(Box::new(RemoteActions::new())),
        306 => Some(Box::new(ButtonEvent::new())),
        307 => Some(Box::new(LcdControl::new())),
        308 => Some(Box::new(PowerOperation::new())),
        309 => Some(Box::new(PowerChannelControl::new())),
        310 => Some(Box::new(QueryPowerChannelState::new())),
        311 => Some(Box::new(PowerChannelState::new())),
        312 => Some(Box::new(LedBrightness::new())),
        313 => Some(Box::new(QueryLedBrightness::new())),
        314 => Some(Box::new(SetLedBrightness::new())),
        315 => Some(Box::new(SetPWM::new())),
        316 => Some(Box::new(PWM::new())),
        350 => Some(Box::new(EstimatedState::new())),
        351 => Some(Box::new(EstimatedStreamVelocity::new())),
        352 => Some(Box::new(IndicatedSpeed::new())),
        353 => Some(Box::new(TrueSpeed::new())),
        354 => Some(Box::new(NavigationUncertainty::new())),
        355 => Some(Box::new(NavigationData::new())),
        356 => Some(Box::new(GpsFixRejection::new())),
        357 => Some(Box::new(LblRangeAcceptance::new())),
        358 => Some(Box::new(DvlRejection::new())),
        360 => Some(Box::new(LblEstimate::new())),
        361 => Some(Box::new(AlignmentState::new())),
        362 => Some(Box::new(GroupStreamVelocity::new())),
        363 => Some(Box::new(Airflow::new())),
        400 => Some(Box::new(DesiredHeading::new())),
        401 => Some(Box::new(DesiredZ::new())),
        402 => Some(Box::new(DesiredSpeed::new())),
        403 => Some(Box::new(DesiredRoll::new())),
        404 => Some(Box::new(DesiredPitch::new())),
        405 => Some(Box::new(DesiredVerticalRate::new())),
        406 => Some(Box::new(DesiredPath::new())),
        407 => Some(Box::new(DesiredControl::new())),
        408 => Some(Box::new(DesiredHeadingRate::new())),
        409 => Some(Box::new(DesiredVelocity::new())),
        410 => Some(Box::new(PathControlState::new())),
        411 => Some(Box::new(AllocatedControlTorques::new())),
        412 => Some(Box::new(ControlParcel::new())),
        413 => Some(Box::new(Brake::new())),
        414 => Some(Box::new(DesiredLinearState::new())),
        415 => Some(Box::new(DesiredThrottle::new())),
        450 => Some(Box::new(Goto::new())),
        451 => Some(Box::new(PopUp::new())),
        452 => Some(Box::new(Teleoperation::new())),
        453 => Some(Box::new(Loiter::new())),
        454 => Some(Box::new(IdleManeuver::new())),
        455 => Some(Box::new(LowLevelControl::new())),
        456 => Some(Box::new(Rows::new())),
        457 => Some(Box::new(FollowPath::new())),
        458 => Some(Box::new(PathPoint::new())),
        459 => Some(Box::new(YoYo::new())),
        460 => Some(Box::new(TeleoperationDone::new())),
        461 => Some(Box::new(StationKeeping::new())),
        462 => Some(Box::new(Elevator::new())),
        463 => Some(Box::new(FollowTrajectory::new())),
        464 => Some(Box::new(TrajectoryPoint::new())),
        465 => Some(Box::new(CustomManeuver::new())),
        466 => Some(Box::new(VehicleFormation::new())),
        467 => Some(Box::new(VehicleFormationParticipant::new())),
        468 => Some(Box::new(StopManeuver::new())),
        469 => Some(Box::new(RegisterManeuver::new())),
        470 => Some(Box::new(ManeuverControlState::new())),
        471 => Some(Box::new(FollowSystem::new())),
        472 => Some(Box::new(CommsRelay::new())),
        473 => Some(Box::new(CoverArea::new())),
        474 => Some(Box::new(PolygonVertex::new())),
        475 => Some(Box::new(CompassCalibration::new())),
        476 => Some(Box::new(FormationParameters::new())),
        477 => Some(Box::new(FormationPlanExecution::new())),
        478 => Some(Box::new(FollowReference::new())),
        479 => Some(Box::new(Reference::new())),
        480 => Some(Box::new(FollowRefState::new())),
        481 => Some(Box::new(FormationMonitor::new())),
        482 => Some(Box::new(RelativeState::new())),
        483 => Some(Box::new(Dislodge::new())),
        484 => Some(Box::new(Formation::new())),
        485 => Some(Box::new(Launch::new())),
        486 => Some(Box::new(Drop::new())),
        487 => Some(Box::new(ScheduledGoto::new())),
        488 => Some(Box::new(RowsCoverage::new())),
        489 => Some(Box::new(Sample::new())),
        490 => Some(Box::new(ImageTracking::new())),
        491 => Some(Box::new(Takeoff::new())),
        492 => Some(Box::new(Land::new())),
        493 => Some(Box::new(AutonomousSection::new())),
        494 => Some(Box::new(FollowPoint::new())),
        495 => Some(Box::new(Alignment::new())),
        496 => Some(Box::new(StationKeepingExtended::new())),
        497 => Some(Box::new(ManeuverDone::new())),
        499 => Some(Box::new(Magnetometer::new())),
        500 => Some(Box::new(VehicleState::new())),
        501 => Some(Box::new(VehicleCommand::new())),
        502 => Some(Box::new(MonitorEntityState::new())),
        503 => Some(Box::new(EntityMonitoringState::new())),
        504 => Some(Box::new(OperationalLimits::new())),
        505 => Some(Box::new(GetOperationalLimits::new())),
        506 => Some(Box::new(Calibration::new())),
        507 => Some(Box::new(ControlLoops::new())),
        508 => Some(Box::new(VehicleMedium::new())),
        509 => Some(Box::new(Collision::new())),
        510 => Some(Box::new(FormState::new())),
        511 => Some(Box::new(AutopilotMode::new())),
        512 => Some(Box::new(FormationState::new())),
        513 => Some(Box::new(ReportControl::new())),
        514 => Some(Box::new(StateReport::new())),
        515 => Some(Box::new(TransmissionRequest::new())),
        516 => Some(Box::new(TransmissionStatus::new())),
        517 => Some(Box::new(SmsRequest::new())),
        518 => Some(Box::new(SmsStatus::new())),
        519 => Some(Box::new(VtolState::new())),
        520 => Some(Box::new(ArmingState::new())),
        521 => Some(Box::new(TCPRequest::new())),
        522 => Some(Box::new(TCPStatus::new())),
        525 => Some(Box::new(AssetReport::new())),
        550 => Some(Box::new(Abort::new())),
        551 => Some(Box::new(PlanSpecification::new())),
        552 => Some(Box::new(PlanManeuver::new())),
        553 => Some(Box::new(PlanTransition::new())),
        554 => Some(Box::new(EmergencyControl::new())),
        555 => Some(Box::new(EmergencyControlState::new())),
        556 => Some(Box::new(PlanDB::new())),
        557 => Some(Box::new(PlanDBState::new())),
        558 => Some(Box::new(PlanDBInformation::new())),
        559 => Some(Box::new(PlanControl::new())),
        560 => Some(Box::new(PlanControlState::new())),
        561 => Some(Box::new(PlanVariable::new())),
        562 => Some(Box::new(PlanGeneration::new())),
        563 => Some(Box::new(LeaderState::new())),
        564 => Some(Box::new(PlanStatistics::new())),
        600 => Some(Box::new(ReportedState::new())),
        601 => Some(Box::new(RemoteSensorInfo::new())),
        602 => Some(Box::new(Map::new())),
        603 => Some(Box::new(MapFeature::new())),
        604 => Some(Box::new(MapPoint::new())),
        606 => Some(Box::new(CcuEvent::new())),
        650 => Some(Box::new(VehicleLinks::new())),
        651 => Some(Box::new(TrexObservation::new())),
        652 => Some(Box::new(TrexCommand::new())),
        655 => Some(Box::new(TrexOperation::new())),
        656 => Some(Box::new(TrexAttribute::new())),
        657 => Some(Box::new(TrexToken::new())),
        658 => Some(Box::new(TrexPlan::new())),
        660 => Some(Box::new(Event::new())),
        702 => Some(Box::new(CompressedImage::new())),
        703 => Some(Box::new(ImageTxSettings::new())),
        750 => Some(Box::new(RemoteState::new())),
        800 => Some(Box::new(Target::new())),
        801 => Some(Box::new(EntityParameter::new())),
        802 => Some(Box::new(EntityParameters::new())),
        803 => Some(Box::new(QueryEntityParameters::new())),
        804 => Some(Box::new(SetEntityParameters::new())),
        805 => Some(Box::new(SaveEntityParameters::new())),
        806 => Some(Box::new(CreateSession::new())),
        807 => Some(Box::new(CloseSession::new())),
        808 => Some(Box::new(SessionSubscription::new())),
        809 => Some(Box::new(SessionKeepAlive::new())),
        810 => Some(Box::new(SessionStatus::new())),
        811 => Some(Box::new(PushEntityParameters::new())),
        812 => Some(Box::new(PopEntityParameters::new())),
        813 => Some(Box::new(IoEvent::new())),
        814 => Some(Box::new(UamTxFrame::new())),
        815 => Some(Box::new(UamRxFrame::new())),
        816 => Some(Box::new(UamTxStatus::new())),
        817 => Some(Box::new(UamRxRange::new())),
        818 => Some(Box::new(UamTxRange::new())),
        820 => Some(Box::new(FormCtrlParam::new())),
        821 => Some(Box::new(FormationEval::new())),
        822 => Some(Box::new(FormationControlParams::new())),
        823 => Some(Box::new(FormationEvaluation::new())),
        850 => Some(Box::new(SoiWaypoint::new())),
        851 => Some(Box::new(SoiPlan::new())),
        852 => Some(Box::new(SoiCommand::new())),
        853 => Some(Box::new(SoiState::new())),
        877 => Some(Box::new(MessagePart::new())),
        888 => Some(Box::new(NeptusBlob::new())),
        889 => Some(Box::new(Aborted::new())),
        890 => Some(Box::new(UsblAngles::new())),
        891 => Some(Box::new(UsblPosition::new())),
        892 => Some(Box::new(UsblFix::new())),
        893 => Some(Box::new(ParametersXml::new())),
        894 => Some(Box::new(GetParametersXml::new())),
        895 => Some(Box::new(SetImageCoords::new())),
        896 => Some(Box::new(GetImageCoords::new())),
        897 => Some(Box::new(GetWorldCoordinates::new())),
        898 => Some(Box::new(UsblAnglesExtended::new())),
        899 => Some(Box::new(UsblPositionExtended::new())),
        900 => Some(Box::new(UsblFixExtended::new())),
        901 => Some(Box::new(UsblModem::new())),
        902 => Some(Box::new(UsblConfig::new())),
        903 => Some(Box::new(DissolvedOrganicMatter::new())),
        904 => Some(Box::new(OpticalBackscatter::new())),
        905 => Some(Box::new(Tachograph::new())),
        906 => Some(Box::new(ApmStatus::new())),
        907 => Some(Box::new(SadcReadings::new())),
        908 => Some(Box::new(DmsDetection::new())),
        909 => Some(Box::new(HomePosition::new())),
        1014 => Some(Box::new(CurrentProfile::new())),
        1015 => Some(Box::new(CurrentProfileCell::new())),
        1016 => Some(Box::new(ADCPBeam::new())),
        2000 => Some(Box::new(GpioState::new())),
        2001 => Some(Box::new(GpioStateGet::new())),
        2002 => Some(Box::new(GpioStateSet::new())),
        2003 => Some(Box::new(ColoredDissolvedOrganicMatter::new())),
        2004 => Some(Box::new(FluorescentDissolvedOrganicMatter::new())),
        2006 => Some(Box::new(TotalMagIntensity::new())),
        2010 => Some(Box::new(CommRestriction::new())),
        2011 => Some(Box::new(FlightEvent::new())),
        2012 => Some(Box::new(Scalar::new())),
        _ => None,
    }
}
