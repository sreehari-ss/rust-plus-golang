extern crate handlebars;
extern crate serde_json;
use handlebars::{handlebars_helper, Handlebars};
use serde_json::Value;
use std::ffi::c_char;
use std::{
    ffi::{CStr, CString},
    str::FromStr,
};


handlebars_helper!(isdefined: |v: Value| !v.is_null());


#[unsafe(no_mangle)]
pub extern "C" fn render_template(template: *const c_char,data: *const c_char) -> *mut c_char {
    let template = unsafe { CStr::from_ptr(template) };
    let template = template.to_str().unwrap();
   
    let data = unsafe { CStr::from_ptr(data) };
    let data = data.to_str().unwrap();

    let mut reg = Handlebars::new();

    reg.register_helper("isdefined", Box::new(isdefined));

     reg.register_template_string("tpl_1", template)
     .unwrap();

    let json:Value = serde_json::from_str(data).unwrap();
    let ret = reg.render("tpl_1", &json).unwrap();

    CString::from_str(&ret).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn free_rust_string(name: *mut c_char)  {
    unsafe { 
        drop(CString::from_raw(name)) };
}


// This is present so it's easy to test that the code works natively in Rust via `cargo test`
#[cfg(test)]
pub mod test {
    use super::*;
    use std::ffi::CString; // This is meant to do the same stuff as the main function in the .go files

    #[test]
    fn test_render_template() {
        render_template(CString::new("Good afternoon, {{name}}, isDefined {{ isdefined age}}").unwrap().into_raw(),CString::new(r#"{"name":"hari"}"#).unwrap().into_raw());
    }
}

