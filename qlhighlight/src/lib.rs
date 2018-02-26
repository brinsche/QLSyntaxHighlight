extern crate core_foundation;
extern crate core_foundation_sys;
extern crate syntect;

use std::ffi::CString;
use core_foundation::url::CFURL;
use core_foundation_sys::url::CFURLRef;

use std::fmt::Write;

use core_foundation::base::TCFType;

use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_snippet_for_file;

#[no_mangle]
pub extern "C" fn highlight_html(url: CFURLRef) -> CString {
    let url = unsafe { CFURL::wrap_under_get_rule(url) };
    let path = url.to_path().unwrap();

    let mut buffer = String::new();

    let ss = SyntaxSet::load_defaults_nonewlines();
    let ts = ThemeSet::load_defaults();

    let theme = &ts.themes["base16-ocean.dark"];
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    write!(
        buffer,
        "<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
        c.r, c.g, c.b
    );
    let html = highlighted_snippet_for_file(path, &ss, theme).unwrap();
    write!(buffer, "{}", html);
    write!(buffer, "{}", "</body>");

    CString::new(buffer).unwrap()
}
