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
#![allow(non_upper_case_globals)]

use std::ffi::CStr;
use std::os::raw::{c_uchar, c_char, c_uint, c_int};
use mirsdrapi_rsp_sys::*;


/* Deprecated API functions */

#[test]
#[ignore]
fn init() {
    assert!(true);
}

#[test]
#[ignore]
fn uninit() {
    assert!(true);
}

#[test]
#[ignore]
fn read_packet() {
    assert!(true);
}

#[test]
#[ignore]
fn set_param() {
    assert!(true);
}

#[test]
#[ignore]
fn get_gr_by_freq() {
    assert!(true);
}

/* END Deprecated API functions */


#[test]
fn reinit() {
}

#[test]
fn set_rf() {
}

#[test]
fn set_fs() {
}

#[test]
fn set_gr() {
}

#[test]
fn set_gr_params() {
}

#[test]
fn set_dc_mode() {
}

#[test]
fn set_dc_track_time() {
}

#[test]
fn set_sync_update_sample_num() {
}

#[test]
fn set_sync_update_period() {
}

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
fn reset_update_flags() {
}

#[test]
fn set_transfer_mode() {
}

#[test]
fn down_convert() {
}

#[test]
fn set_ppm() {
}

#[test]
fn set_lo_mode() {
}

#[test]
fn set_gr_alt_mode() {
}

#[test]
fn dc_offset_iq_imbalance_control() {
}

#[test]
fn decimate_control() {
}

#[test]
fn agc_control() {
}

#[test]
fn stream_init() {
}

#[test]
fn stream_uninit() {
}

#[test]
fn debug_enable() {
}

#[test]
fn get_current_gain() {
}

#[test]
fn gain_change_callback_message_received() {
}

fn _get_devices() -> Result<Vec<mir_sdr_DeviceT>, &'static str> {
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
    match err_return {
        mir_sdr_ErrT_mir_sdr_Success => {},
        _ => {return Err("mir_sdr_GetDevices() failed.");},
    }
    devices.truncate(num_dev as usize);
    match devices.len() {
        x if x > 0 => {return Ok(devices);},
        _ => {return Err("Device not found. Please ensure device is connected.");},
    }
}

#[test]
fn get_devices() {
    if let Ok(devices) = _get_devices() {
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
    } else {
        panic!("Test \"get_devices()\" failed.")
    }
}

#[test]
fn set_device_idx() {
}

#[test]
fn release_device_idx() {
}

#[test]
fn get_hw_version() {
}


/* RSP2 */

#[test]
fn rsp2_antenna_control() {
}

#[test]
fn rsp2_external_reference_control() {
}

#[test]
fn rsp2_biasT_control() {
}

#[test]
fn rsp2_rf_notch_enable() {
}

#[test]
fn rsp_set_gr() {
}

#[test]
fn rsp_set_gr_limits() {
}

#[test]
fn am_port_select() {
}


/* RSP 1a */

#[test]
fn rsp1a_biastT() {
}

#[test]
fn rsp1a_dab_notch() {
}

#[test]
fn rsp1a_broadcast_notch() {
}


/* RSPduo */

#[test]
fn rspduo_tuner_select() {
}

#[test]
fn rspduo_ext_ref() {
}

#[test]
fn rspduo_biasT() {
}

#[test]
fn rspduo_tuner1_am_notch() {
}

#[test]
fn rspduo_broadcast_notch() {
}

#[test]
fn rspduo_dab_notch() {
}
