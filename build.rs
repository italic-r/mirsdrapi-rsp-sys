use pkg_config;
use bindgen;
use std::env;
use std::path::PathBuf;

fn main() -> () {
    let crate_root = env!("CARGO_MANIFEST_DIR");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .use_core()
        .generate_block(true)
        .header("include/mirsdrapi-rsp.h")

        // blacklist all function prototypes except init callbacks
        .blacklist_type("mir_sdr_Init_t")
        .blacklist_type("mir_sdr_Uninit_t")
        .blacklist_type("mir_sdr_ReadPacket_t")
        .blacklist_type("mir_sdr_SetRf_t")
        .blacklist_type("mir_sdr_SetFs_t")
        .blacklist_type("mir_sdr_SetGr_t")
        .blacklist_type("mir_sdr_SetGrParams_t")
        .blacklist_type("mir_sdr_SetDcMode_t")
        .blacklist_type("mir_sdr_SetDcTrackTime_t")
        .blacklist_type("mir_sdr_SetSyncUpdateSampleNum_t")
        .blacklist_type("mir_sdr_SetSyncUpdatePeriod_t")
        .blacklist_type("mir_sdr_ApiVersion_t")
        .blacklist_type("mir_sdr_ResetUpdateFlags_t")

        .blacklist_type("mir_sdr_SetJavaReqCallback_t")

        .blacklist_type("mir_sdr_SetTransferMode_t")
        .blacklist_type("mir_sdr_DownConvert_t")
        .blacklist_type("mir_sdr_SetParam_t")
        .blacklist_type("mir_sdr_SetPpm_t")
        .blacklist_type("mir_sdr_SetLoMode_t")
        .blacklist_type("mir_sdr_SetGrAltMode_t")
        .blacklist_type("mir_sdr_DCoffsetIQimbalanceControl_t")
        .blacklist_type("mir_sdr_DecimateControl_t")
        .blacklist_type("mir_sdr_AgcControl_t")
        .blacklist_type("mir_sdr_StreamInit_t")
        .blacklist_type("mir_sdr_StreamUninit_t")
        .blacklist_type("mir_sdr_Reinit_t")
        .blacklist_type("mir_sdr_GetGrByFreq_t")
        .blacklist_type("mir_sdr_DebugEnable_t")
        .blacklist_type("mir_sdr_GetCurrentGain_t")
        .blacklist_type("mir_sdr_GainChangeCallbackMessageReceived_t")

        .blacklist_type("mir_sdr_GetDevices_t")
        .blacklist_type("mir_sdr_SetDeviceIdx_t")
        .blacklist_type("mir_sdr_ReleaseDeviceIdx_t")
        .blacklist_type("mir_sdr_GetHwVersion_t")
        .blacklist_type("mir_sdr_RSPII_AntennaControl_t")
        .blacklist_type("mir_sdr_RSPII_ExternalReferenceControl_t")
        .blacklist_type("mir_sdr_RSPII_BiasTControl_t")
        .blacklist_type("mir_sdr_RSPII_RfNotchEnable_t")

        .blacklist_type("mir_sdr_RSP_SetGr_t")
        .blacklist_type("mir_sdr_RSP_SetGrLimits_t")

        .blacklist_type("mir_sdr_AmPortSelect_t")

        .blacklist_type("mir_sdr_rsp1a_BiasT_t")
        .blacklist_type("mir_sdr_rsp1a_DabNotch_t")
        .blacklist_type("mir_sdr_rsp1a_BroadcastNotch_t")

        .blacklist_type("mir_sdr_rspDuo_TunerSel_t")
        .blacklist_type("mir_sdr_rspDuo_ExtRef_t")
        .blacklist_type("mir_sdr_rspDuo_BiasT_t")
        .blacklist_type("mir_sdr_rspDuo_Tuner1AmNotch_t")
        .blacklist_type("mir_sdr_rspDuo_BroadcastNotch_t")
        .blacklist_type("mir_sdr_rspDuo_DabNotch_t")

        .generate()
        .unwrap();

    bindings.write_to_file(out_dir.join("bindings.rs"))
        .expect("Cannot write bindings.");

    // only use pkg_config ON linux TARGETTING linux
    #[cfg(unix)]
    {
        if cfg!(target_os = "linux") {
            let pkg = PathBuf::from(&crate_root).join("pkgconfig");
            let _env = env::set_var("PKG_CONFIG_PATH", &pkg);
            pkg_config::Config::new()
                .atleast_version("2.13.1")
                .probe("mirsdrapi-rsp").expect("Cannot find RSP driver.");
        }
    }
}
