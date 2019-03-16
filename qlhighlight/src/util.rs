use crate::quicklook::CFLog;
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFType, TCFType, ToVoid};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};
use core_foundation::string::{CFString, CFStringRef};
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;
use syntect::highlighting::{Color, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;

extern "C" {
    pub fn CFPreferencesCopyMultiple(
        keysToFetch: CFArrayRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> CFDictionaryRef;

    #[link_name = "kCFPreferencesCurrentUser"]
    pub static kCFPreferencesCurrentUser: CFStringRef;
    #[link_name = "kCFPreferencesAnyHost"]
    pub static kCFPreferencesAnyHost: CFStringRef;
}

pub const DEFAULT_THEME_NAME: &str = "Xcode-like";
const DEFAULT_FONT_FAMILY: &str = "Menlo, monospace";
const DEFAULT_FONT_SIZE: &str = "11";

const FONT_SIZE: &str = "fontSize";
const FONT_FAMILY: &str = "fontFamily";
const THEME: &str = "theme";
const THEME_DIR: &str = "themeDirectory";
const SYNTAX_DIR: &str = "syntaxDirectory";

pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

#[derive(Debug)]
pub struct Config {
    pub font_size: CFString,
    pub font_family: CFString,
    pub syntax_set: SyntaxSet,
    pub theme: Theme,
}

pub fn get_settings() -> Config {
    let keys = CFArray::from_CFTypes(&[
        CFString::new(FONT_SIZE),
        CFString::new(FONT_FAMILY),
        CFString::new(THEME),
        CFString::new(THEME_DIR),
        CFString::new(SYNTAX_DIR),
    ]);

    let prefs = unsafe {
        CFPreferencesCopyMultiple(
            keys.as_concrete_TypeRef(),
            CFString::new("de.bastianrinsche.QLSyntaxHighlight").as_concrete_TypeRef(),
            kCFPreferencesCurrentUser,
            kCFPreferencesAnyHost, // TODO: why any??
        )
    };

    let prefs: CFDictionary = unsafe { CFDictionary::wrap_under_get_rule(prefs) };

    let font_size = prefs
        .find(CFString::from_static_string(FONT_SIZE).to_void())
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(*ptr).downcast::<CFString>() })
        .unwrap_or_else(|| CFString::new(DEFAULT_FONT_SIZE));

    let font_family = prefs
        .find(CFString::from_static_string(FONT_FAMILY).to_void())
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(*ptr).downcast::<CFString>() })
        .unwrap_or_else(|| CFString::from_static_string(DEFAULT_FONT_FAMILY));

    let theme_name = prefs
        .find(CFString::from_static_string(THEME).to_void())
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(*ptr).downcast::<CFString>() })
        .unwrap_or_else(|| CFString::from_static_string(DEFAULT_THEME_NAME));

    let theme_dir = prefs
        .find(CFString::from_static_string(THEME_DIR).to_void())
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(*ptr).downcast::<CFString>() });

    let syntax_dir = prefs
        .find(CFString::from_static_string(SYNTAX_DIR).to_void())
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(*ptr).downcast::<CFString>() });

    let mut theme_set = ThemeSet::load_defaults();
    let mut syntax_builder = SyntaxSet::load_defaults_nonewlines().into_builder();

    if let Some(theme_dir) = theme_dir {
        let directory = theme_dir.to_string();
        let theme_dir = Path::new(&directory);
        theme_set
            .add_from_folder(&theme_dir)
            .unwrap_or_else(|e| log(&e.to_string()));
    };

    if let Some(syntax_dir) = syntax_dir {
        let directory = syntax_dir.to_string();
        let syntax_dir = Path::new(&directory);
        syntax_builder
            .add_from_folder(&Path::new(&syntax_dir), false)
            .unwrap_or_else(|e| log(&e.to_string()));
    };

    let syntax_set = syntax_builder.build();

    let theme_bytes = include_bytes!("../res/XCodelike.tmTheme");
    let xcode_theme = ThemeSet::load_from_reader(&mut Cursor::new(&theme_bytes[..])).unwrap();

    let theme = theme_set
        .themes
        .get(&theme_name.to_string())
        .unwrap_or(&xcode_theme)
        .clone();

    Config {
        font_size,
        font_family,
        theme,
        syntax_set,
    }
}

pub fn read_file_to_string(file_path: &Path) -> Result<Vec<u8>, ::std::io::Error> {
    let mut content: Vec<u8> = Vec::new();
    let mut file = File::open(&file_path)?;
    file.read_to_end(&mut content)?;
    Ok(content)
}

pub fn log(log: &str) {
    unsafe {
        CFLog(0, CFString::new(log));
    }
}
