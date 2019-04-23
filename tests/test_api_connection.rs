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

use std::{thread, time};
use std::slice;
use std::ffi::CStr;
use std::os::raw::{c_uchar, c_char, c_uint, c_int, c_short, c_void};
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
fn reinit() {
    /* does not require reinit:
     * GR mode
     * rf frequency
     *
     * requires reinit:
     * sample rate
     * bandwidth type
     * if type
     * lo mode
     * am port
     */

    if let Ok(devs) = _get_devices() {
        if let Err(e) = _set_device_idx(devs, 0) {
            panic!(e);
        }
    }
    if let Err(c) = _stream_init() {
        if let Err(e) = _release_device_idx() {
            panic!(e);
        }
        panic!(c);
    }

    // give enough time for device to initialize and call callback
    let sleep_time = time::Duration::from_millis(50);
    thread::sleep(sleep_time);

    let mut gRdB: c_int = 0;
    let mut gRdBsystem: c_int = 0;
    let mut samplesPerPacket: c_int = 0;

    let reinit_err_return = unsafe {
        mir_sdr_Reinit(
            &mut gRdB,
            2.,
            7.15,
            mir_sdr_Bw_MHzT_mir_sdr_BW_0_300,
            mir_sdr_If_kHzT_mir_sdr_IF_Zero,
            mir_sdr_LoModeT_mir_sdr_LO_Auto,
            4, // LNAmode
            &mut gRdBsystem,
            mir_sdr_SetGrModeT_mir_sdr_USE_SET_GR,
            &mut samplesPerPacket,
            mir_sdr_ReasonForReinitT_mir_sdr_CHANGE_BW_TYPE,
        )
    };

    // give enough time for device to initialize and call callback
    let sleep_time = time::Duration::from_millis(50);
    thread::sleep(sleep_time);

    match reinit_err_return {
        mir_sdr_ErrT_mir_sdr_Success       => {
            match _stream_uninit() {
                Ok(_) => {
                    if let Err(e) = _release_device_idx() {
                        panic!(e);
                    }
                },
                Err(c) => panic!(c),
            }
        },
        mir_sdr_ErrT_mir_sdr_InvalidParam  => panic!("NULL pointers."),
        mir_sdr_ErrT_mir_sdr_OutOfRange    => panic!("Requested parameters outside of acceptable range."),
        mir_sdr_ErrT_mir_sdr_AliasingError => panic!("Aliasing error."),
        mir_sdr_ErrT_mir_sdr_HwError       => panic!("Could not access hardware."),
        mir_sdr_ErrT_mir_sdr_Fail          => panic!("Other failure."),
        _                                  => unreachable!(),
    }
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

unsafe extern "C" fn _stream_callback(
    xi: *mut c_short, // array of i16
    xq: *mut c_short, // array of i16
    firstSampleNum: c_uint,
    grChanged: c_int,
    rfChanged: c_int,
    fsChanged: c_int,
    numSamples: c_uint,
    reset: c_uint,
    hwRemoved: c_uint,
    cbContext: *mut c_void // opaque struct: rust -> C -> rust
) {
    assert!(!xi.is_null());
    assert!(!xq.is_null());
    let xi = slice::from_raw_parts(xi, numSamples as usize);
    let xq = slice::from_raw_parts(xq, numSamples as usize);
    println!("IQ:             {:#?}, {:#?}", xi[0], xq[0]);
    println!("firstSampleNum: {:#?}", firstSampleNum);
    println!("grChanged:      {:#?}", grChanged);
    println!("rfChanged:      {:#?}", rfChanged);
    println!("fsChanged:      {:#?}", fsChanged);
    println!("numSamples:     {:#?}", numSamples);
    println!("reset:          {:#?}", reset);
    println!("hwRemoved:      {:#?}", hwRemoved);
    println!("cbContext:      {:#?}", cbContext);
}

unsafe extern "C" fn _gain_change_callback(
    gRdB: c_uint,
    lnaGRdB: c_uint,
    cbContext: *mut c_void
) {
    println!("gRdB:           {:#?}", gRdB);
    println!("lnaGRdB:        {:#?}", lnaGRdB);
    println!("cbContext:      {:#?}", cbContext);
}

#[repr(C)]
struct ContextObject {
    _private: u32,
}

fn _stream_init() -> Result<(), &'static str>{
    // unsafe {mir_sdr_DebugEnable(1)};

    let mut gRdb: i32 = 0;
    let mut gRdBsystem: c_int = 0;
    let mut samplesPerPacket: c_int = 1024;
    let mut _cbContext = ContextObject {_private: 2};
    // WTF is this
    let cbContext: *mut c_void = &mut _cbContext as *mut _ as *mut c_void;

    let err_return = unsafe {
        mir_sdr_StreamInit(
            &mut gRdb,
            2., // fsMHz
            7.15, // rfMHz
            mir_sdr_Bw_MHzT_mir_sdr_BW_0_600, // bwType
            mir_sdr_If_kHzT_mir_sdr_IF_Zero, // ifType
            4, // LNAstate
            &mut gRdBsystem,
            mir_sdr_SetGrModeT_mir_sdr_USE_SET_GR, // setGrMode
            &mut samplesPerPacket,
            Some(_stream_callback),
            Some(_gain_change_callback),
            cbContext,
        )
    };
    match err_return {
        mir_sdr_ErrT_mir_sdr_Success            => Ok(()),
        mir_sdr_ErrT_mir_sdr_AlreadyInitialised => Err("API already initialized."),
        mir_sdr_ErrT_mir_sdr_InvalidParam       => Err("Null pointers."),
        mir_sdr_ErrT_mir_sdr_OutOfRange         => Err("Parameters out of range."),
        mir_sdr_ErrT_mir_sdr_HwError            => Err("Failed to access device."),
        mir_sdr_ErrT_mir_sdr_Fail               => Err("Other failure."),
        _                                       => unreachable!(),
    }

}

#[test]
fn stream_init() {
    if let Ok(devs) = _get_devices() {
        if let Err(e) = _set_device_idx(devs, 0) {
            panic!(e);
        }
    }
    let err_return = _stream_init();

    // give enough time for device to initialize and call callback
    let sleep_time = time::Duration::from_millis(50);
    thread::sleep(sleep_time);

    match err_return {
        Ok(_) => {
            if let Err(c) = _stream_uninit() {
                panic!(c);
            } else {
                if let Err(e) = _release_device_idx() {
                    panic!(e);
                }
            }
        },
        Err(c) => panic!(c),
    }
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
    if let Err(c) = _stream_uninit() {
        panic!(c);
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
fn get_hw_version() {
    let mut ver: c_uchar = 0;

    if let Ok(devices) = _get_devices() {
        if let Ok(_) = _set_device_idx(devices, 0) {
            // must initialize stream before calling this function
            if let Ok(_) = _stream_init() {
                // give enough time for device to initialize and call callback
                let sleep_time = time::Duration::from_millis(50);
                thread::sleep(sleep_time);

                if let mir_sdr_ErrT_mir_sdr_Success = unsafe {mir_sdr_GetHwVersion(&mut ver)} {
                    assert!(
                        ver == test_dev_hw_ver,
                        format!("Device versions do not match; returned: {}, test hardware: {}",
                                &ver, &test_dev_hw_ver));
                }

                if let Err(c) = _stream_uninit() {
                    panic!(c);
                }
                if let Err(c) = _release_device_idx() {
                    panic!(c);
                }
            }
        }
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
