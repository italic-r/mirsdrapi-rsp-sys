#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for mir_sdr_DeviceT {
    fn default() -> Self {
        mir_sdr_DeviceT {
            SerNo: &mut 0,
            DevNm: &mut 0,
            hwVer: 0,
            devAvail: 1,
        }
    }
}
