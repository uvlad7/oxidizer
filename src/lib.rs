mod oxide;

use oxide::{oxy_function, wrap_oxyfunction};
use oxide::{oxy_init, OxyModule, OxyResult, Python};

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
pub fn get_tzs(_py: Python, lng: f64, lat: f64) -> OxyResult<Vec<&str>> {
    Ok(FINDER.get_tz_names(lng, lat))
}

// #[oxy_function]
// pub fn timezonenames(_py: Python) -> OxyResult<Vec<&str>> {
//     return Ok(FINDER.timezonenames());
// }

// Test raw values are supported as well
#[oxy_function]
pub fn timezonenames(_py: Python) -> Vec<&str> {
    FINDER.timezonenames()
}

#[oxy_function]
pub fn data_version(_py: Python) -> OxyResult<String> {
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
