#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate uorb_codec;
use uorb_codec::common::*;
use uorb_codec::UorbMsgMeta;




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

/// Serialize one message type
fn ser_sensor_gyro() {
    let msg_data = get_sensor_gyro();
    for _i in 0..100 {
        let _encoded: Vec<u8> = msg_data.ser();
        //assert_ne!(msg_data.len(), 0);
    }
}

/// Deserialize one message type
fn deser_sensor_gyro() {
    let msg_data = get_sensor_gyro();
    let encoded:Vec<u8> = msg_data.ser();
    let sliced_encoded = encoded.as_slice();
    for _i in 0..100 {
        let decoded = SensorGyroData::deser(sliced_encoded).unwrap();
        assert_eq!(msg_data.timestamp, decoded.timestamp);
    }
}


fn criterion_serialization_benchmark(c: &mut Criterion) {
    c.bench_function("serialization", |b| b.iter(|| ser_sensor_gyro()));
}
fn criterion_deser_benchmark(c: &mut Criterion) {
    c.bench_function("deserialization", |b| b.iter(|| deser_sensor_gyro()));
}


criterion_group!(benches, criterion_serialization_benchmark, criterion_deser_benchmark);
criterion_main!(benches);