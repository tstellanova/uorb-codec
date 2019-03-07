
extern crate uorb_codec;

pub mod test_shared;


mod test_serialization {
    use crate::test_shared;
    use uorb_codec::common::*;
    use uorb_codec::UorbMsgMeta;

    #[test]
    pub fn test_deserialize_actuator_controls() {
        let msg_data = test_shared::get_actuator_controls();
        let encoded:Vec<u8> = msg_data.ser();
        let decoded = ActuatorControlsData::deser(encoded.as_slice()).unwrap();
        assert_eq!(msg_data.timestamp, decoded.timestamp);
    }

    #[test]
    pub fn test_deserialize_vehicle_status() {
        let msg_data = test_shared::get_vehicle_status();
        let encoded:Vec<u8> = msg_data.ser();
        let decoded = VehicleStatusData::deser(encoded.as_slice()).unwrap();
        assert_eq!(msg_data.onboard_control_sensors_health, decoded.onboard_control_sensors_health);
    }

    #[test]
    pub fn test_deserialize_sensor_gyro() {
        let msg_data = test_shared::get_sensor_gyro();
        let encoded:Vec<u8> = msg_data.ser();
        let decoded = SensorGyroData::deser(encoded.as_slice()).unwrap();
        assert_eq!(msg_data.timestamp, decoded.timestamp);
    }


}

