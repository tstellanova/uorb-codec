
extern crate uorb_codec;

mod test_shared;


#[cfg(test)]
mod test_encode_decode {
    use crate::test_shared;
    use uorb_codec::common::*;
    use uorb_codec::UorbMsgMeta;


    #[test]
    pub fn test_deserialize_actuator_controls() {
        let msg = test_shared::get_actuator_controls();
        let encoded:Vec<u8> = msg.ser();
        let decoded = ActuatorControlsData::deser(encoded.as_slice()).unwrap();
        assert_eq!(msg.timestamp, decoded.timestamp);
    }

    #[test]
    pub fn test_deserialize_vehicle_status() {
        let msg = test_shared::get_vehicle_status();
        let encoded:Vec<u8> = msg.ser();
        let decoded = VehicleStatusData::deser(encoded.as_slice()).unwrap();
        assert_eq!(msg.onboard_control_sensors_health, decoded.onboard_control_sensors_health);
    }

    #[test]
    pub fn test_write_vehicle_status() {
        let mut v = vec![];
        let msg_data = test_shared::get_vehicle_status();
        let mut header = uorb_codec::UorbHeader {
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


    }
}

