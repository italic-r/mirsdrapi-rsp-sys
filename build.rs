use pkg_config;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), pkg_config::Error> {
    let crate_root = env!("CARGO_MANIFEST_DIR");
    let pkg = PathBuf::from(&crate_root).join("pkgconfig");
    let _env = env::set_var("PKG_CONFIG_PATH", &pkg);

    pkg_config::Config::new().atleast_version("2.13.1").probe("mirsdrapi-rsp")?;

    Ok(())
}
