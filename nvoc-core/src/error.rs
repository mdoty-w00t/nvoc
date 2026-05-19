use quick_error::quick_error;
use std::io;
use std::num::{ParseFloatError, ParseIntError};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Nvapi(err: nvapi_hi::Error) {from()source(err)display("NVAPI error: {}", err)}
        VfpUnsupported {display("VFP unsupported")}
        DeviceNotFound {display("no matching device found")}
        Io(err: io::Error) {from()source(err)display("IO error: {}", err)}
        ParseInt(err: ParseIntError) {from()source(err)display("{}", err)}
        ParseFloat(err: ParseFloatError) {from()source(err)display("{}", err)}
        Str(err: &'static str) {from()display("{}", err)}
        FeatureUnsupportedErr{display("Feature unsupported")}
        Custom(err: String) { from() display("{}", err) }  // Corrected syntax
    }
}

impl From<nvapi_hi::NvapiError> for Error {
    fn from(e: nvapi_hi::NvapiError) -> Self {
        Self::from(nvapi_hi::Error::from(e))
    }
}
