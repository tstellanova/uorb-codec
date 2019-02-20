extern crate uorb_codec;

mod test_shared;

use uorb_codec::common::*;
use uorb_codec::UorbMsgMeta;

#[cfg(test)]
#[test]
pub fn test_write_read_vehicle_status() {
    let mut v = vec![];
    let msg_data = test_shared::get_vehicle_status();
    let header = uorb_codec::UorbHeader {
        version: uorb_codec::UORB_MAGIC_V1,
        hash: VehicleStatusData::MSG_HASH_CODE,
        instance_id: 0,
        payload_len: VehicleStatusData::ENCODED_LEN,
    };
    let msg = UorbMessage::VehicleStatus(msg_data);

    uorb_codec::write_msg(
        &mut v,
        &header,
        &msg,
    ).expect("Failed to write message");

    let mut c = v.as_slice();
    let (decoded_header, decoded_msg) = uorb_codec::read_msg(&mut c).expect("Failed to read");

    if let uorb_codec::common::UorbMessage::VehicleStatus(decoded_msg) = decoded_msg {
        assert_eq!(decoded_header.hash, header.hash);
        assert_eq!(decoded_msg.timestamp, msg_data.clone().timestamp);
    } else {
        panic!("Read wrong message type");
    }



}