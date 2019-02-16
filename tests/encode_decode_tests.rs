

extern crate uorb_codec;



#[cfg(test)]
mod test_encode_decode {
    #[test]
    pub fn test_create_actuator_controls() {
        let foo = uorb_codec::common::ActuatorControls {
            timestamp: 19,
            timestamp_sample: 21
        };

        assert_eq!(foo.timestamp, 19);
    }
}

