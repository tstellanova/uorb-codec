
extern crate uorb_codec;
use uorb_codec::common::*;

pub fn get_vehicle_gps_position() -> VehicleGpsPositionData {
    VehicleGpsPositionData {
        timestamp: 83838333,
        lat: 1005,
        lon: 3355,
        alt: 500,
        alt_ellipsoid: 500,
        s_variance_m_s: 1.1,
        c_variance_rad: 0.5,
        fix_type: 3,
        eph: 1.0,
        epv: 1.0,
        hdop: 2.0,
        vdop: 3.0,
        noise_per_ms: 5,
        jamming_indicator: 0,
        vel_m_s: 0.001,
        vel_n_m_s: 0.001,
        vel_e_m_s: 0.001,
        vel_d_m_s: 0.001,
        cog_rad: 0.01,
        vel_ned_valid: true,
        timestamp_time_relative: 55,
        time_utc_usec: 83838333,
        satellites_used: 11,
        heading: 0.01,
        heading_offset: 0.0,
    }
}

pub fn get_vehicle_status() -> VehicleStatusData {
    VehicleStatusData {
        timestamp: 83838333,
        nav_state: 14,
        arming_state: 15,
        hil_state: 16,
        failsafe: false,
        system_type: 99,
        system_id: 51,
        component_id: 49,
        is_rotary_wing: true,
        is_vtol: true,
        vtol_fw_permanent_stab: true,
        in_transition_mode: false,
        in_transition_to_fw: false,
        rc_signal_lost: false,
        rc_input_mode: 12,
        data_link_lost: false,
        high_latency_data_link_active: false,
        data_link_lost_counter: 0,
        engine_failure: false,
        mission_failure: false,
        failure_detector_status: 0,
        onboard_control_sensors_present: 12345,
        onboard_control_sensors_enabled: 12345,
        onboard_control_sensors_health: 12345,
    }
}

pub fn get_actuator_controls() -> ActuatorControlsData {
    ActuatorControlsData {
        timestamp: 19,
        timestamp_sample: 21,
        control: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
    }
}

const GYRO_REBASE_FACTOR:f32 =  1E3;

pub fn get_sensor_gyro() -> SensorGyroData {
    let xgyro = 1111.0;
    let ygyro = 2222.0;
    let zgyro = 3333.0;

    SensorGyroData {
        device_id: 6887,
        timestamp: 0x1ed59200230008,
        error_count: 0,
        x: xgyro,
        y: ygyro,
        z: zgyro,
        integral_dt: 5 * 1000000,
        x_integral: 1.0,
        y_integral: 1.0,
        z_integral: 1.0,
        temperature: 25.0,
        scaling: 0.0,
        x_raw: (xgyro * GYRO_REBASE_FACTOR) as i16,
        y_raw: (ygyro * GYRO_REBASE_FACTOR) as i16,
        z_raw: (zgyro * GYRO_REBASE_FACTOR) as i16,
    }
}


