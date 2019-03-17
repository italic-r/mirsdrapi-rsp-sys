#![allow(unused_imports)]

use std::ffi::CStr;
use std::os::raw::{c_uchar, c_char, c_uint, c_int};
use mirsdrapi_rsp_sys::*;


#[test]
fn api_version() -> () {
    let mut ver: f32 = 0.;
    unsafe {
        mir_sdr_ApiVersion(&mut ver);
    }
    assert_eq!(ver, 2.13);
    println!("API version: {}", ver);
}

/// Because each device is different, only print device data.
#[test]
fn get_devices() {
    let max_devices: c_uint = 4;  // arbitrary limit
    let mut num_dev: c_uint = 0;

    let dummy = mir_sdr_DeviceT {
        SerNo: &mut 0,
        DevNm: &mut 0,
        hwVer: 0,
        devAvail: 1,
    };

    let mut devices: Vec<mir_sdr_DeviceT> = Vec::with_capacity(max_devices as usize);
    for _d in 0..max_devices {
        devices.push(dummy.clone());
    }

    let err_return = unsafe {
        mir_sdr_GetDevices(
            devices.as_mut_slice().first_mut().unwrap(),
            &mut num_dev,
            max_devices
        )
    };
    assert!(err_return == mir_sdr_ErrT_mir_sdr_Success);
    devices.truncate(num_dev as usize);

    // TEST DEVICE:
    // SerNo: 1809003232
    // DevNm: SDRplay_RSPII_VID_1DF7&PID_3020_BUS_001_PORT_002
    // hwVer: 3

    for dev in &devices {
        println!("device: {:?}", &dev);

        let ser_no = unsafe {CStr::from_ptr(dev.SerNo)};
        let ser_no = ser_no.clone().to_string_lossy().parse::<i32>();
        if let Ok(ser) = ser_no {
            println!("SerNo: {}", &ser);
        } else {
            panic!("Could not retrieve device serial number.");
        }

        let dev_nm = unsafe {CStr::from_ptr(dev.DevNm)}.clone().to_string_lossy();
        println!("DevNm: {}", &dev_nm);

        println!("hwVer: {}", &dev.hwVer);
        println!("devAvail: {}", &dev.devAvail);
    }

}
