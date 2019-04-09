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
#![allow(non_snake_case)]

use std::ffi::CStr;
use std::os::raw::{c_uchar, c_char, c_uint, c_int};
use mirsdrapi_rsp_sys::*;

const test_dev_hw_ver: c_uchar = 3;


/* Deprecated API functions */

#[test]
#[ignore]
fn init() {
    panic!();
}

#[test]
#[ignore]
fn uninit() {
    panic!();
}

#[test]
#[ignore]
fn read_packet() {
    panic!();
}

#[test]
#[ignore]
fn set_param() {
    panic!();
}

#[test]
#[ignore]
fn get_gr_by_freq() {
    panic!();
}

/* END Deprecated API functions */


#[test]
#[ignore]
fn reinit() {
}

#[test]
#[ignore]
fn set_rf() {
}

#[test]
#[ignore]
fn set_fs() {
}

#[test]
#[ignore]
fn set_gr() {
}

#[test]
#[ignore]
fn set_gr_params() {
}

#[test]
#[ignore]
fn set_dc_mode() {
}

#[test]
#[ignore]
fn set_dc_track_time() {
}

#[test]
#[ignore]
fn set_sync_update_sample_num() {
}

#[test]
#[ignore]
fn set_sync_update_period() {
}

#[test]
fn api_version() {
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
#[ignore]
fn reset_update_flags() {
}

#[test]
#[ignore]
fn set_transfer_mode() {
}

#[test]
#[ignore]
fn down_convert() {
}

#[test]
fn set_ppm() {
    match unsafe {mir_sdr_SetPpm(0.)} {
        mir_sdr_ErrT_mir_sdr_Success => {},
        mir_sdr_ErrT_mir_sdr_RfUpdateError => {
            panic!("Previously requested update has not yet been applied.");
        },
        mir_sdr_ErrT_mir_sdr_OutOfRange => {
            panic!("Requested PPM value out of range.");
        },
        mir_sdr_ErrT_mir_sdr_HwError => {
            panic!("Error accessing device.");
        },
        _ => unreachable!(),
    }
}

#[test]
#[ignore]
fn set_lo_mode() {
}

#[test]
#[ignore]
fn set_gr_alt_mode() {
}

#[test]
#[ignore]
fn dc_offset_iq_imbalance_control() {
}

#[test]
#[ignore]
fn decimate_control() {
}

#[test]
#[ignore]
fn agc_control() {
}

#[test]
#[ignore]
fn stream_init() {
}

fn _stream_uninit() -> Result<(), &'static str> {
    match unsafe {mir_sdr_StreamUninit()} {
        mir_sdr_ErrT_mir_sdr_Success => Ok(()),
        mir_sdr_ErrT_mir_sdr_Fail => Err("Stream uninit failed."),
        _ => unreachable!(),
    }
}

#[test]
#[ignore]
fn stream_uninit() {
    match _stream_uninit() {
        Ok(()) => {},
        Err(c) => panic!(c),
    }
}

#[test]
fn debug_enable() {
    assert!(unsafe {mir_sdr_DebugEnable(1)} == mir_sdr_ErrT_mir_sdr_Success, "Failed to enable debug.");
    assert!(unsafe {mir_sdr_DebugEnable(0)} == mir_sdr_ErrT_mir_sdr_Success, "Failed to disable debug.")
}

#[test]
#[ignore]
fn get_current_gain() {
}

#[test]
#[ignore]
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

    match unsafe {mir_sdr_GetDevices(
            devices.as_mut_slice().first_mut().unwrap(),
            &mut num_dev,
            max_devices)} {
        mir_sdr_ErrT_mir_sdr_Success => {},
        _ => return Err("mir_sdr_GetDevices() failed."),
    }
    devices.truncate(num_dev as usize);
    match devices.len() {
        0 => return Err("Device not found. Please ensure device is connected."),
        _ => return Ok(devices),
    }
}

#[test]
fn get_devices() {
    if let Ok(devices) = _get_devices() {
        for dev in &devices {
            println!("Device: {:?}", &dev);

            let ser_no = unsafe {CStr::from_ptr(dev.SerNo)};
            let ser_no = ser_no.clone().to_string_lossy().parse::<i32>();
            assert!(ser_no.is_ok(), "Could not parse device serial number.");

            let dev_nm = unsafe {CStr::from_ptr(dev.DevNm)}.clone().to_string_lossy();
            println!("DevNm: {}", &dev_nm);

            println!("hwVer: {}", &dev.hwVer);
            println!("devAvail: {}", &dev.devAvail);
        }
    } else {
        panic!();
    }
}

fn _set_device_idx(devices: Vec<mir_sdr_DeviceT>, dev: u32) -> Result<(), &'static str> {
    if dev > devices.len() as u32 {
        return Err("Index of requested device is out of range.");
    }
    match unsafe {mir_sdr_SetDeviceIdx(dev)} {
        mir_sdr_ErrT_mir_sdr_Success => Ok(()),
        mir_sdr_ErrT_mir_sdr_HwError => return Err("Hardware error at lock. Device may be in use."),
        _ => unreachable!(),
    }
}

fn _release_device_idx() -> Result<(), &'static str> {
    match unsafe {mir_sdr_ReleaseDeviceIdx()} {
        mir_sdr_ErrT_mir_sdr_Success => Ok(()),
        mir_sdr_ErrT_mir_sdr_HwError => return Err("Hardware error at release."),
        _ => unreachable!(),
    }
}

#[test]
fn set_device_idx() {
    if let Ok(devices) = _get_devices() {
        assert!(_set_device_idx(devices, 0).is_ok());
    }
    assert!(_release_device_idx().is_ok());
}

#[test]
fn release_device_idx() {
    if let Ok(devices) = _get_devices() {
        assert!(_set_device_idx(devices, 0).is_ok());
    }
    assert!(_release_device_idx().is_ok());
}

#[test]
#[ignore]
fn get_hw_version() {
    // XXX: getting bad HwVer on return

    // must initialize a device before using this function
    let mut ver: c_uchar = 0;
    if let Ok(devices) = _get_devices() {
        if let Ok(()) = _set_device_idx(devices, 0) {
            if let mir_sdr_ErrT_mir_sdr_Success = unsafe {mir_sdr_GetHwVersion(&mut ver)} {
                // HwVer managed by rust, so can release device in case of assert failure.
                if let Ok(()) = _release_device_idx() {
                        assert!(ver == test_dev_hw_ver,
                            format!("\nCurrent hardware version: {}\nTest hardware version: {}\n",
                            &ver, &test_dev_hw_ver));
                }
            } else {
                _release_device_idx();
                panic!("API returned: InvalidParam");
            }
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}


/* RSP2 */

#[test]
#[ignore]
fn rsp2_antenna_control() {
}

#[test]
#[ignore]
fn rsp2_external_reference_control() {
}

#[test]
#[ignore]
fn rsp2_biasT_control() {
}

#[test]
#[ignore]
fn rsp2_rf_notch_enable() {
}

#[test]
#[ignore]
fn rsp_set_gr() {
}

#[test]
#[ignore]
fn rsp_set_gr_limits() {
}

#[test]
#[ignore]
fn am_port_select() {
}


/* RSP 1a */

#[test]
#[ignore]
fn rsp1a_biastT() {
}

#[test]
#[ignore]
fn rsp1a_dab_notch() {
}

#[test]
#[ignore]
fn rsp1a_broadcast_notch() {
}


/* RSPduo */

#[test]
#[ignore]
fn rspduo_tuner_select() {
}

#[test]
#[ignore]
fn rspduo_ext_ref() {
}

#[test]
#[ignore]
fn rspduo_biasT() {
}

#[test]
#[ignore]
fn rspduo_tuner1_am_notch() {
}

#[test]
#[ignore]
fn rspduo_broadcast_notch() {
}

#[test]
#[ignore]
fn rspduo_dab_notch() {
}
