
extern crate uorb_codec;

mod test_shared;


#[cfg(test)]
mod test_encode_decode {
    use crate::test_shared;
    use uorb_codec::common::*;

    #[test]
    pub fn test_encode_decode_actuator_controls() {
        let msg = test_shared::get_actuator_controls();
        let encoded:Vec<u8> = msg.ser();
        let decoded = ActuatorControls::deser(encoded.as_slice()).unwrap();
        assert_eq!(msg.timestamp, decoded.timestamp);
    }

    #[test]
    pub fn test_encode_decode_vehicle_status() {
        let msg = test_shared::get_vehicle_status();
        let encoded:Vec<u8> = msg.ser();
        let decoded = VehicleStatus::deser(encoded.as_slice()).unwrap();
        assert_eq!(msg.onboard_control_sensors_health, decoded.onboard_control_sensors_health);
    }
}

