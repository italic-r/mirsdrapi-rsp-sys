//! SDRplay devices retrieve data from an RF receiver and returns data based on
//! its USB interface, device type, RF reception, etc. Most tests will simply
//! print data, since a lot of data (ie serial number, device name) is unique
//! and not all user devices will be equivalent to the test device. The device
//! used to build this wrapper and associated tests is the RSPduo dual-tuner
//! model. The driver used is provided by SDRplay, version 2.13.
//!
//! Serial number: 1809003232
//! Device name: SDRplay_RSPII_VID_1DF7&PID_3020_BUS_001_PORT_002
//! Hardware version: 3

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
    assert_eq!(
        ver, 2.13,
        "API version not supported. Please install version 2.13 from SDRplay."
    );
}

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
    assert!(
        devices.len() > 0,
        "Did not find devices. Please ensure device is connected."
    );

    for dev in &devices {
        println!("Device: {:?}", &dev);

        let ser_no = unsafe {CStr::from_ptr(dev.SerNo)};
        let ser_no = ser_no.clone().to_string_lossy().parse::<i32>();
        if let Ok(ser) = ser_no {
            println!("SerNo: {}", &ser);
        } else {
            panic!("Could not parse device serial number.");
        }

        let dev_nm = unsafe {CStr::from_ptr(dev.DevNm)}.clone().to_string_lossy();
        println!("DevNm: {}", &dev_nm);

        println!("hwVer: {}", &dev.hwVer);
        println!("devAvail: {}", &dev.devAvail);
    }

}
