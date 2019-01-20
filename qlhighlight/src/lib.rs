mod highlight;
mod quicklook;
mod util;

use core_foundation::base::{OSStatus, TCFType};
use core_foundation::data::CFData;
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::url::{CFURLRef, CFURL};
use highlight::determine_file_type;
use highlight::FileType::*;
use quicklook::kUTTypeHTML;
use quicklook::QLPreviewRequestRef;
use quicklook::QLPreviewRequestSetDataRepresentation;
use util::read_file_to_string;

#[no_mangle]
pub extern "C" fn GeneratePreviewForURL(
    _this_interface: usize, //???
    preview: QLPreviewRequestRef,
    url: CFURLRef,
    content_type_uti: CFStringRef,
    options: CFDictionaryRef,
) -> OSStatus {
    let url = unsafe { CFURL::wrap_under_get_rule(url) };
    let content_type_uti = unsafe { CFString::wrap_under_get_rule(content_type_uti) };

    let path = url.to_path().unwrap();
    let conf = util::get_settings();

    let buffer = match read_file_to_string(&path) {
        Ok(buffer) => match determine_file_type(content_type_uti) {
            Binary => highlight::hex_highlight_file(&buffer, &conf),
            Plist => highlight::highlight_plist(&buffer, &conf),
            Syntax => match highlight::syntax_highlight_file(&buffer, &path, &conf) {
                Ok(html) => html,
                Err(_) => highlight::format_err("Error reading file.", &conf),
            },
        },
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
