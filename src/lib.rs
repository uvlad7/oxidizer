mod oxide;

use oxide::{oxy_function, wrap_oxyfunction};
use oxide::{oxy_init, OxyModule, OxyResult};

// #![allow(unused)]
use lazy_static::lazy_static;
use tzf_rs::DefaultFinder;

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::default();
}

#[oxy_function]
pub fn get_tz(lng: f64, lat: f64) -> OxyResult<String> {
    Ok(FINDER.get_tz_name(lng, lat).to_string())
}

#[oxy_function]
pub fn get_tzs(lng: f64, lat: f64) -> OxyResult<Vec<String>> {
    Ok(FINDER.get_tz_names(lng, lat).into_iter().map(|v| v.to_string()).collect())
}

#[oxy_function]
pub fn timezonenames() -> OxyResult<Vec<String>> {
    return Ok(FINDER.timezonenames().into_iter().map(|v| v.to_string()).collect());
}

#[oxy_function]
pub fn data_version() -> OxyResult<String> {
    return Ok(FINDER.data_version().to_string());
}

#[oxy_init]
fn tzfpy(m: &OxyModule) -> OxyResult<()> {
    m.add_function(wrap_oxyfunction!(get_tz, m)?)?;
    m.add_function(wrap_oxyfunction!(get_tzs, m)?)?;
    m.add_function(wrap_oxyfunction!(timezonenames, m)?)?;
    m.add_function(wrap_oxyfunction!(data_version, m)?)?;
    Ok(())
}
