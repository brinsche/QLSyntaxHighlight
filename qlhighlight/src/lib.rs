extern crate core_foundation;
extern crate syntect;

mod highlight;
mod util;
mod quicklook;

use core_foundation::base::{OSStatus, TCFType};
use core_foundation::data::CFData;
use core_foundation::url::{CFURLRef, CFURL};
use core_foundation::string::CFStringRef;
use core_foundation::dictionary::CFDictionaryRef;

use quicklook::QLPreviewRequestRef;
use quicklook::QLPreviewRequestSetDataRepresentation;
use quicklook::kUTTypeHTML;

#[no_mangle]
pub extern "C" fn GeneratePreviewForURL(
    _this_interface: usize, //???
    preview: QLPreviewRequestRef,
    url: CFURLRef,
    _content_type_uti: CFStringRef,
    options: CFDictionaryRef,
) -> OSStatus {
    let url = unsafe { CFURL::wrap_under_get_rule(url) };
    let path = url.to_path().unwrap();
    let conf = util::get_settings();

    let buffer = match highlight::highlight_file(&path, &conf) {
        Ok(html) => html,
        Err(_) => highlight::format_err("Error reading file.", &conf),
    };

    let data = CFData::from_buffer(buffer.as_bytes());

    unsafe {
        QLPreviewRequestSetDataRepresentation(
            preview,
            data.as_concrete_TypeRef(),
            kUTTypeHTML,
            options,
        )
    };
    0i32
}

#[no_mangle]
pub extern "C" fn CancelPreviewGeneration(
    _this_interface: usize, //???
    _preview: QLPreviewRequestRef,
) {
    // Implement only if supported
}
