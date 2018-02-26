extern crate core_foundation;
extern crate core_foundation_sys;

use std::ffi::CString;
use core_foundation::url::CFURL;
use core_foundation_sys::url::CFURLRef;

use core_foundation::base::TCFType;

use std::io::Read;
use std::fs::File;

#[no_mangle]
pub extern "C" fn highlight_html(url: CFURLRef) -> CString {
    let url = unsafe { CFURL::wrap_under_get_rule(url) };
    let path = url.to_path().unwrap();

    let mut f = File::open(path).unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).unwrap();

    let html = format!("<html><body>{}</body></html>", buffer);
    CString::new(html).unwrap()
}
