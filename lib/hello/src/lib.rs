extern crate handlebars;
extern crate serde_json;
use core::{error, fmt, str};
use std::fmt::write;
use handlebars::{Handlebars, RenderError, handlebars_helper};
use lazy_static::lazy_static;
use serde_json::{Error as SerdeJsonError, Value};
use std::ffi::c_char;
use std::str::Utf8Error;
use std::{
    ffi::{CStr, CString, NulError},
    str::FromStr,
};

lazy_static! {
    pub static ref reg: Handlebars<'static> = {
        let mut hb = Handlebars::new();
        hb.register_helper("isdefined", Box::new(isdefined));
        hb.set_strict_mode(true);
        hb
    };
}

handlebars_helper!(isdefined: |v: Value| !v.is_null());

#[derive(Debug)]
enum HBError {
    Utf8Error(str::Utf8Error),
    SerdeJsonError(SerdeJsonError),
    NulError(NulError),
    RenderError(RenderError),

}

impl fmt::Display for HBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     let errstr =    match self {
            HBError::Utf8Error(utf8_error) => utf8_error.to_string(),
            HBError::SerdeJsonError(error) => error.to_string(),
            HBError::NulError(nul_error) => nul_error.to_string(),
            HBError::RenderError(render_error) => render_error.to_string(),

        };
        write!(f,"{}",errstr )
    }
}

impl error::Error for HBError {}

impl From<str::Utf8Error> for HBError {
    fn from(value: str::Utf8Error) -> Self {
        HBError::Utf8Error(value)
    }
}

impl From<SerdeJsonError> for HBError {
    fn from(value: SerdeJsonError) -> Self {
        HBError::SerdeJsonError(value)
    }
}

impl From<NulError> for HBError {
    fn from(value: NulError) -> Self {
        HBError::NulError(value)
    }
}

impl From<RenderError> for HBError {
    fn from(value: RenderError) -> Self {
        HBError::RenderError(value)
    }
}

struct FFIOut {
    raw_ptr: *mut c_char,
}

impl FFIOut {
    fn new(raw_ptr: *mut c_char) -> Self {
        FFIOut { raw_ptr }
    }
}

impl From<Result<String, HBError>> for FFIOut {
    fn from(value: Result<String, HBError>) -> Self {
        let res: Result<CString, HBError> =
            value.and_then(|value| {
                let output = format!(r#"{{"value":"{}"}}"#,value);
                 CString::from_str(&output).map_err(|err| err.into())
                });
        match res {
            Ok(cstr) => Self::new(cstr.into_raw()),
            Err(err) => {
                let err_string = format!(r#"{{"error":"{}"}}"#,&err.to_string() );
                // it's okay to unwrap in here hoping error string wouldn't contain a null characters
                let err_string = CString::from_str(&err_string).unwrap();
                FFIOut::new(err_string.into_raw())
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn render_template(template: *const c_char, data: *const c_char) -> *mut c_char {
    let template = unsafe { CStr::from_ptr(template) };

    let data = unsafe { CStr::from_ptr(data) };

    let res = render_template_inner(template, data);
    FFIOut::from(res).raw_ptr
}

fn render_template_inner(template: &CStr, data: &CStr) -> Result<String, HBError> {
    let template = template.to_str()?;
    let data = data.to_str()?;
    let json: Value = serde_json::from_str(data)?;
    reg.render_template(template, &json)
        .map_err(|err| err.into())
}

#[unsafe(no_mangle)]
pub extern "C" fn free_rust_string(name: *mut c_char) {
    unsafe { drop(CString::from_raw(name)) };
}

// This is present so it's easy to test that the code works natively in Rust via `cargo test`
#[cfg(test)]
pub mod test {
    use super::*;
    use std::ffi::CString; // This is meant to do the same stuff as the main function in the .go files

    #[test]
    fn test_render_template() {
        render_template(
            CString::new("Good afternoon, {{name}}")
                .unwrap()
                .into_raw(),
            CString::new(r#"{"name":"hari"}"#).unwrap().into_raw(),
        );
    }

    #[test]
    fn test_render_template_strictmode() {
        render_template(
            CString::new("Good afternoon, {{name}}, isDefined {{ isdefined age}}")
                .unwrap()
                .into_raw(),
            CString::new(r#"{"name":"hari"}"#).unwrap().into_raw(),
        );
    }
}
