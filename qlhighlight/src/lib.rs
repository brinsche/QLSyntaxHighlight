extern crate core_foundation;
extern crate core_foundation_sys;
extern crate syntect;

mod util;

use std::fmt::Write;
use std::io::Cursor;

use core_foundation::base::TCFType;
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::url::CFURL;
use core_foundation_sys::url::CFURLRef;
use core_foundation::string::CFStringRef;
use core_foundation::dictionary::CFDictionaryRef;

use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_snippet_for_file;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __QLPreviewRequest {
    _unused: [u8; 0],
}
pub type Boolean = ::std::os::raw::c_uchar;
pub type OSStatus = ::std::os::raw::c_int;
pub type QLPreviewRequestRef = *mut __QLPreviewRequest;

extern "C" {
    #[link_name = "kUTTypeHTML"]
    pub static kUTTypeHTML: CFStringRef;
    #[link_name = "kUTTypePlainText"]
    pub static kUTTypePlainText: CFStringRef;
    pub fn QLPreviewRequestSetDataRepresentation(
        preview: QLPreviewRequestRef,
        data: CFDataRef,
        contentTypeUTI: CFStringRef,
        properties: CFDictionaryRef,
    );
    pub fn QLPreviewRequestIsCancelled(preview: QLPreviewRequestRef) -> Boolean;
}

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
    let theme_bytes = include_bytes!("../res/XCodelike.tmTheme");
    let xcode_theme = ThemeSet::load_from_reader(&mut Cursor::new(&theme_bytes[..])).unwrap();

    let ts = ThemeSet::load_defaults();
    let syntax_set = SyntaxSet::load_defaults_nonewlines();
    let theme = ts.themes
        .get(&conf.theme_name.to_string())
        .unwrap_or(&xcode_theme);

    let style = format!(
        "pre {{ font-size: {}px; font-family: {}; }}",
        conf.font_size, conf.font_family
    );
    let c = theme.settings.background.unwrap_or(Color::WHITE);

    let mut buffer = String::new();
    write!(
        buffer,
        "<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
        c.r, c.g, c.b
    );
    write!(buffer, "<head><style>{}</style></head>", style);
    let html = highlighted_snippet_for_file(path, &syntax_set, theme).unwrap();
    write!(buffer, "{}", html);

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
