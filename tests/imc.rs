use bytes::{BufMut, IntoBuf};
use crc16::{State, ARC};
use imc::packet::{crc, deserializeHeader};
use imc::Alignment::Alignment;
use imc::EstimatedState::EstimatedState;
use imc::EstimatedStreamVelocity::EstimatedStreamVelocity;
use imc::FollowReference::FollowReference;
use imc::GpsFix::{GpsFix, ValidityEnum};
use imc::Header::Header;
use imc::Heartbeat::Heartbeat;
use imc::LoggingControl::{ControlOperationEnum, LoggingControl};
use imc::Loiter::Loiter;
use imc::Message::Message;
use imc::NavigationUncertainty::NavigationUncertainty;
use imc::PlanManeuver::PlanManeuver;
use imc::PlanSpecification::PlanSpecification;
use imc::PlanTransition::PlanTransition;
use imc::PlanVariable::PlanVariable;
use imc::VehicleCommand::VehicleCommand;
use imc::{IMC_CONST_FOOTER_SIZE, IMC_CONST_HEADER_SIZE};

#[test]
fn buffer_write() {
    let mut bfr = bytes::BytesMut::with_capacity(1024);
    bfr.put("IMC RUST");

    assert_eq!(bfr[0], b'I');
    // make sure it doesn't advance cursor
    assert_ne!(bfr[0], b'M');
    assert_eq!(bfr.len(), 8);

    bfr.put("look at me testing");
    assert_eq!(bfr[0], b'I');
    assert_eq!(bfr[8], b'l');
    assert_eq!(bfr[8..].len(), b"look at me testing".len());
}

#[test]
fn imc_crc() {
    let mut lc = LoggingControl::new();
    lc.set_timestamp_secs(0.23424);
    lc.set_source(765);
    lc.set_source_ent(230);
    lc.set_destination(57);
    lc.set_destination_ent(32);
    lc._name = String::from("20210707_IMC_RUST_TEST");
    lc._op = ControlOperationEnum::COP_REQUEST_START.value();

    let mut bfr: bytes::BytesMut = bytes::BytesMut::with_capacity(lc.serialization_size());
    let ret = imc::packet::serialize(&mut lc, &mut bfr);
    assert!(ret.is_ok());
    assert_eq!(ret.ok().unwrap(), lc.serialization_size());

    let crc = crc::compute_from_range(
        0,
        lc.serialization_size() - IMC_CONST_FOOTER_SIZE as usize,
        &mut bfr,
    );

    assert_eq!(crc.get(), 1427_u16);
}

#[test]
fn header_serialization() {
    let mut hdr: Header = Header::new(1_u16);
    hdr._timestamp = 0.143432;
    hdr._size = 42;
    hdr._src = 205;
    hdr._src_ent = 2;
    hdr._dst = 300;
    hdr._dst_ent = 10;

    let mut bfr: bytes::BytesMut = bytes::BytesMut::with_capacity(IMC_CONST_HEADER_SIZE as usize);
    hdr.serialize(&mut bfr);

    let mut hdr2: Header = Header::new(0);
    let inbfr = bytes::Bytes::from(bfr);
    let ret = deserializeHeader(&mut hdr2, &mut inbfr.into_buf());

    assert!(ret.is_ok());
    assert_eq!(hdr, hdr2);
}

#[test]
fn deserialize_as() {
    let mut lc = LoggingControl::new();
    lc.set_timestamp_secs(0.23424);
    lc.set_source(765);
    lc.set_source_ent(230);
    lc.set_destination(57);
    lc.set_destination_ent(32);
    lc._name = String::from("20210707_IMC_RUST_TEST");
    lc._op = ControlOperationEnum::COP_REQUEST_START.value();

    let mut bfr: bytes::BytesMut = bytes::BytesMut::with_capacity(lc.serialization_size());

    let ret = imc::packet::serialize(&mut lc, &mut bfr);
    assert!(ret.is_ok());
    assert_eq!(ret.ok().unwrap(), lc.serialization_size());

    let inbfr = bytes::Bytes::from(bfr);
    let ret = imc::packet::deserialize_as::<LoggingControl>(&mut inbfr.into_buf());
    assert!(ret.is_ok());

    let mut lc2 = ret.ok().unwrap();
    assert_eq!(lc.get_header(), lc2.get_header());
    assert_eq!(lc._op, lc2._op);
    assert_eq!(lc._name, lc2._name);
}

#[test]
fn generic_deserialize() {
    let mut lc = LoggingControl::new();
    lc.set_timestamp_secs(0.23424);
    lc.set_source(765);
    lc.set_source_ent(230);
    lc.set_destination(57);
    lc.set_destination_ent(32);
    lc._name = String::from("20210707_IMC_RUST_TEST");
    lc._op = ControlOperationEnum::COP_REQUEST_START.value();

    let mut bfr: bytes::BytesMut = bytes::BytesMut::with_capacity(lc.serialization_size());

    let ret = imc::packet::serialize(&mut lc, &mut bfr);
    assert!(ret.is_ok());
    assert_eq!(ret.ok().unwrap(), lc.serialization_size());

    let inbfr = bytes::Bytes::from(bfr);
    let ret = imc::packet::deserialize(&mut inbfr.into_buf());
    assert!(ret.is_ok());

    let mut lc2 = ret.ok().unwrap();
    assert_eq!(lc.get_header(), lc2.get_header());
}

#[test]
fn test_PlanSpecification() {
    let mut msg: PlanSpecification = PlanSpecification::new();
    msg.set_timestamp_secs(0.06636732601718875);
    msg.set_source(9519);
    msg.set_source_ent(29);
    msg.set_destination(15466);
    msg.set_destination_ent(6);
    msg._plan_id = String::from("BWWQBHFVCWVLKOKGECDOPRYIWFHUZHBSFUEWUYGXBJKPVJZTUDVDIDSWISYGOHIDTUHKOFNNARETZOJLNXGAFDAVMQCKFAYGNDPQPZ");
    msg._description = String::from("DYNLKRVLJIDTKJZKCTYBLSRIYXTNQAAOHMBFWFXDETHGELXZALWHODCWGISMIJPTCYWPTFUWWDAFDVMNCMJNAEKZCGRWGQZWMSVHPXHLPNUFICFMSRZJGSZGRNXPEODOMBESORUVYQCGARHPIOKHULQPJ");
    msg._vnamespace = String::from("WSYMUYGJWNOCTGGGOXDXLOPNRLKAPLELQCYDOCAHQWYWRVNLAVHPHJMFKVFJSJEWNVGYFTCENSTQUZMORQIXUPDSDPCTXUJVAEXSZXCTLBBGKKGBQTHHGIXOIPQ");

    assert_eq!(msg.serialization_size(), 418);

    let mut tmp_msg_0: PlanVariable = PlanVariable::new();
    tmp_msg_0._name = String::from("DXZCRTGUWDWHTWQSERRRSGIEFUKYKLPPASHRDTHOGOISXBBXITVXTJPNOXVVRWPASDUZPOBGHPIXLVXIAYCWEJJJQHMIIDKXSEONFYGQCKABAEUWALSNYKGRIVNCOPNUMVOLMCYAQGPUMMWBWFQJOLFFFKHLTOGNTBHXYLJBUYVQCMQSYLJLAUJEIMHCTLCZVBWFWRDFVBKGUNEVAPHDZSKDENZDTJHDIYFBNTRKC");
    tmp_msg_0._value = String::from("GMTFDDVKMPHVVLMGQQGAPOKWOODZXROVEOB");
    tmp_msg_0._type = 89;
    tmp_msg_0._access = 191;

    assert_eq!(tmp_msg_0.dynamic_serialization_size(), 272);
    assert_eq!(tmp_msg_0.serialization_size(), 296);

    msg._variables.push(tmp_msg_0);
    msg._start_man_id = String::from("JQVWYJUZZGKFDBTWYCSUBTDWKXJRVLWMDQKJJGOQMLGZKPNLODMWYMSZHHDXROTLCEBMQTSBSSHFXLHAKIZJMZYWGZOHYEEKHAQMRQUDWIWFFJQRPOYQEVDTBNINVPVZMIVNCGRMYRGTAFHEGBTUUNERASESFPAXI");

    assert_eq!(msg.dynamic_serialization_size(), 833);
    assert_eq!(msg.serialization_size(), 855);

    let mut tmp_msg_1: PlanManeuver = PlanManeuver::new();
    tmp_msg_1._maneuver_id = String::from("UTZALDQEMLMERGXJFNVNGIETDTROSAODAKWTSXLCHOOOAFYXFZADGQBLNFFZMJPSIUDBMYCGTOQYVCSABBKUIDIJAFHPXPPYGNHSQMJEMFZWCZHYIIRITUAMKNBGQVLZRY");

    assert_eq!(tmp_msg_1.dynamic_serialization_size(), 138);
    assert_eq!(tmp_msg_1.serialization_size(), 160);

    let mut tmp_tmp_msg_1_0 = Loiter::new();
    tmp_tmp_msg_1_0._timeout = 43469;
    tmp_tmp_msg_1_0._lat = 0.5076048407317348;
    tmp_tmp_msg_1_0._lon = 0.8378009613232282;
    tmp_tmp_msg_1_0._z = 0.569276149649857;
    tmp_tmp_msg_1_0._z_units = 157;
    tmp_tmp_msg_1_0._duration = 56167;
    tmp_tmp_msg_1_0._speed = 0.0688217054567426;
    tmp_tmp_msg_1_0._speed_units = 77;
    tmp_tmp_msg_1_0._type = 139;
    tmp_tmp_msg_1_0._radius = 0.04252198382186334;
    tmp_tmp_msg_1_0._length = 0.5792016731405074;
    tmp_tmp_msg_1_0._bearing = 0.42425983894180663;
    tmp_tmp_msg_1_0._direction = 74;
    tmp_tmp_msg_1_0._custom = String::from("OJEVFYRGFVKBQJQXBRPUXIWIRYRLYCMZOUSRQAYUHDMBVVSWQECNWCYUPGVRDBJZHNAPVNOISMHTMWPUBVNEEGYNYDOZOSXCEAEQEBQBNGLGEJMGHEHQKXUMHLZAAKBSKNJSJSBOUQTO");

    assert_eq!(tmp_tmp_msg_1_0.dynamic_serialization_size(), 142);
    assert_eq!(tmp_tmp_msg_1_0.serialization_size(), 212);

    tmp_msg_1._data = Some(Box::new(tmp_tmp_msg_1_0));

    assert_eq!(tmp_msg_1.dynamic_serialization_size(), 328);
    assert_eq!(tmp_msg_1.serialization_size(), 350);

    let mut tmp_tmp_msg_1_1 = Heartbeat::new();

    assert_eq!(tmp_tmp_msg_1_1.dynamic_serialization_size(), 0);
    assert_eq!(tmp_tmp_msg_1_1.serialization_size(), 22);

    tmp_msg_1._end_actions.push(Box::new(tmp_tmp_msg_1_1));

    msg._maneuvers.push(tmp_msg_1);

    assert_eq!(msg.dynamic_serialization_size(), 1165);
    assert_eq!(msg.serialization_size(), 1187);

    let mut tmp_msg_2 = PlanTransition::new();
    tmp_msg_2._source_man = String::from("SBYWHSJIKLTYZQMKHOPDGQTGLLWZKCJHWELYSBPRUGUOFMATIXRSTMDYCIVUIRVHWVIQTXYYOQMAVHMCFSGPWJQDULBPGXNPTPZKIOJQJNMXFFUNEBBRFCDOAUZHVNGMFXJCXFWUIDSQWVRXSDXOEQHGFAWZOEZKYBGNEMSTEAURACNJFRAUFVLEYHRWZWZPACRCLOGZVLAVJMTPKJDBDONRICKJNXGEESENQHDAMZ");
    tmp_msg_2._dest_man = String::from("PGROZASPFDVQAVHXLAPXEFXQBLFKIYRFWZYCKDKHSXUJOYUDXBNMTDTSSBCCLFRCEYWDJDXVNJMACOSMCGJATMVIHVNXGRPCSQSQZJBUOZINRZOFGEBQATLOTRTEYEWNXJOGCNRHVLTELJWKMXVG");
    tmp_msg_2._conditions = String::from("FZREAPDIGIRXNVOKPXKHDHWAOCUMNQROHRLZJWYVGIFOGTXVDDSAGUBYJYOWNAHNEPISKJLPLNNCNRSFMXHJLGYIVNYEAZTPHZY");

    let mut tmp_tmp_msg_2_0 = FollowReference::new();
    tmp_tmp_msg_2_0._control_src = 60688;
    tmp_tmp_msg_2_0._control_ent = 72;
    tmp_tmp_msg_2_0._timeout = 0.9027662940527639;
    tmp_tmp_msg_2_0._loiter_radius = 0.8635480860563065;
    tmp_tmp_msg_2_0._altitude_interval = 0.9260483211923134;
    tmp_msg_2._actions.push(Box::new(tmp_tmp_msg_2_0));
    msg._transitions.push(tmp_msg_2);

    let mut tmp_msg_3 = NavigationUncertainty::new();
    tmp_msg_3._x = 0.7790642103610969;
    tmp_msg_3._y = 0.05912914760218102;
    tmp_msg_3._z = 0.7018785723267326;
    tmp_msg_3._phi = 0.8663628852520322;
    tmp_msg_3._theta = 0.17960330137675773;
    tmp_msg_3._psi = 0.5061622991200995;
    tmp_msg_3._p = 0.03693987453118697;
    tmp_msg_3._q = 0.48669094286208914;
    tmp_msg_3._r = 0.002271736811042646;
    tmp_msg_3._u = 0.004317411102729318;
    tmp_msg_3._v = 0.6204113959344103;
    tmp_msg_3._w = 0.6986172922693936;
    tmp_msg_3._bias_psi = 0.7629508231548181;
    tmp_msg_3._bias_r = 0.1725182683049824;
    msg._start_actions.push(Box::new(tmp_msg_3));

    let mut tmp_msg_4 = EstimatedStreamVelocity::new();
    tmp_msg_4._x = 0.39923077397445994;
    tmp_msg_4._y = 0.1659079132718545;
    tmp_msg_4._z = 0.40359083865499956;
    msg._end_actions.push(Box::new(tmp_msg_4));

    assert_eq!(1779, msg.serialization_size());
    let mut bfr: bytes::BytesMut = bytes::BytesMut::with_capacity(msg.serialization_size());

    let ret = imc::packet::serialize(&mut msg, &mut bfr);
    assert!(ret.is_ok());
    assert_eq!(ret.ok().unwrap(), msg.serialization_size());
}

#[test]
fn test_PlanTransition() {
    let mut msg = PlanTransition::new();
    msg._source_man = String::from("SBYWHSJIKLTYZQMKHOPDGQTGLLWZKCJHWELYSBPRUGUOFMATIXRSTMDYCIVUIRVHWVIQTXYYOQMAVHMCFSGPWJQDULBPGXNPTPZKIOJQJNMXFFUNEBBRFCDOAUZHVNGMFXJCXFWUIDSQWVRXSDXOEQHGFAWZOEZKYBGNEMSTEAURACNJFRAUFVLEYHRWZWZPACRCLOGZVLAVJMTPKJDBDONRICKJNXGEESENQHDAMZ");
    msg._dest_man = String::from("PGROZASPFDVQAVHXLAPXEFXQBLFKIYRFWZYCKDKHSXUJOYUDXBNMTDTSSBCCLFRCEYWDJDXVNJMACOSMCGJATMVIHVNXGRPCSQSQZJBUOZINRZOFGEBQATLOTRTEYEWNXJOGCNRHVLTELJWKMXVG");
    msg._conditions = String::from("FZREAPDIGIRXNVOKPXKHDHWAOCUMNQROHRLZJWYVGIFOGTXVDDSAGUBYJYOWNAHNEPISKJLPLNNCNRSFMXHJLGYIVNYEAZTPHZY");

    let mut msg1 = FollowReference::new();
    msg1._control_src = 60688;
    msg1._control_ent = 72;
    msg1._timeout = 0.9027662940527639;
    msg1._loiter_radius = 0.8635480860563065;
    msg1._altitude_interval = 0.9260483211923134;
    msg._actions.push(Box::new(msg1));

    assert_eq!(msg.dynamic_serialization_size(), 506);
    assert_eq!(msg.serialization_size(), 528);

    let mut bfr: bytes::BytesMut = bytes::BytesMut::with_capacity(msg.serialization_size());

    let ret = imc::packet::serialize(&mut msg, &mut bfr);
    assert!(ret.is_ok());
    assert_eq!(ret.ok().unwrap(), msg.serialization_size());
}

#[test]
fn test_VehicleCommand() {
    let mut msg: VehicleCommand = VehicleCommand::new();
    msg.set_timestamp_secs(0.3256085741897383);
    msg.set_source(55716);
    msg.set_source_ent(122);
    msg.set_destination(36257);
    msg.set_destination_ent(166);
    msg._type = 27;
    msg._request_id = 31451;
    msg._command = 91;

    assert_eq!(msg.dynamic_serialization_size(), 4);
    assert_eq!(msg.serialization_size(), 32);

    let mut tmp_msg_0: Alignment = Alignment::new();
    tmp_msg_0._timeout = 49484;
    tmp_msg_0._lat = 0.6773872429275245;
    tmp_msg_0._lon = 0.9711109462825693;
    tmp_msg_0._speed = 0.9415963179494303;
    tmp_msg_0._speed_units = 127;
    tmp_msg_0._custom = String::from("AXPZXRJRIRYJDOTDHPKSNOXZJTYCUHIMUPKIXGBGUWSOGRGLUFTLPWPIR");

    assert_eq!(tmp_msg_0.dynamic_serialization_size(), 59);
    assert_eq!(tmp_msg_0.serialization_size(), 104);

    msg._maneuver = Option::Some(Box::new(tmp_msg_0));
    msg._calib_time = 61097;
    msg._info = String::from("PYOMQYPTDKBVVXAKSFWGBURXTJNDVXJFVRFOSDXJBIDOFGRSSTAIEBWSMMJPZJGZVFOHIHITMDUZGKQWTYZCACVMPCOLBHLCEUHBODRQAXGRYUNBXDGQGWHZMSQJMNIJZCNHVVQEBPBLKWIARKLLHAYSVSEFYRKJNNCXKQEITXNZAHLMVMUCPPEYFONF");

    assert_eq!(msg.dynamic_serialization_size(), 274);
    assert_eq!(msg.serialization_size(), 302);

    let mut bfr: bytes::BytesMut = bytes::BytesMut::with_capacity(msg.serialization_size());

    let ret = imc::packet::serialize(&mut msg, &mut bfr);
    assert!(ret.is_ok());
    assert_eq!(ret.ok().unwrap(), msg.serialization_size());

    let inbfr = bytes::Bytes::from(bfr);
    let ret = imc::packet::deserialize(&mut inbfr.into_buf());
    assert!(ret.is_ok());

    let mut msg2 = ret.ok().unwrap();
    assert_eq!(msg.get_header(), msg2.get_header());
}

#[test]
pub fn test_GpsFix() {
    let mut fix: GpsFix = GpsFix::new();
    fix._validity = (ValidityEnum::GFV_VALID_COG as u16) | (ValidityEnum::GFV_VALID_HACC as u16);

    let mut ret = 0;
    ret = fix._validity & (ValidityEnum::GFV_VALID_COG as u16);

    assert_ne!(ret, 0);
}
