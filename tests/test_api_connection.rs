use mirsdrapi_rsp_sys::*;


#[test]
fn api_version() -> () {
    let mut ver: f32 = 0.;
    unsafe {
        mir_sdr_ApiVersion(&mut ver);
    }
    assert_eq!(ver, 2.13);
}
