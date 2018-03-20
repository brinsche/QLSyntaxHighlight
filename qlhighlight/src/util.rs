use std::io::Cursor;
use std::path::Path;

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFType, TCFType};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};

use syntect::highlighting::{Theme, ThemeSet};
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

pub const DEFAULT_THEME_NAME: &'static str = "Xcode-like";
const DEFAULT_FONT_FAMILY: &'static str = "Menlo, monospace";
const DEFAULT_FONT_SIZE: &'static str = "11";

const FONT_SIZE: &'static str = "fontSize";
const FONT_FAMILY: &'static str = "fontFamily";
const THEME: &'static str = "theme";
const THEME_DIR: &'static str = "themeDirectory";
const SYNTAX_DIR: &'static str = "syntaxDirectory";

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
        .find2(&CFString::new(FONT_SIZE))
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(ptr).downcast::<CFString>() })
        .unwrap_or(CFString::new(DEFAULT_FONT_SIZE));

    let font_family = prefs
        .find2(&CFString::new(FONT_FAMILY))
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(ptr).downcast::<CFString>() })
        .unwrap_or(CFString::new(DEFAULT_FONT_FAMILY));

    let theme_name = prefs
        .find2(&CFString::new(THEME))
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(ptr).downcast::<CFString>() })
        .unwrap_or(CFString::new(DEFAULT_THEME_NAME));

    let theme_dir = prefs
        .find2(&CFString::new(THEME_DIR))
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(ptr).downcast::<CFString>() });

    let syntax_dir = prefs
        .find2(&CFString::new(SYNTAX_DIR))
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(ptr).downcast::<CFString>() });

    let mut theme_set = ThemeSet::load_defaults();
    let mut syntax_set = SyntaxSet::load_defaults_nonewlines();

    if let Some(theme_dir) = theme_dir {
        let directory = theme_dir.to_string();
        let theme_dir = Path::new(&directory);
        if let Ok(mut custom_themes) = ThemeSet::load_from_folder(&theme_dir) {
            theme_set.themes.append(&mut custom_themes.themes);
        }
    };

    if let Some(syntax_dir) = syntax_dir {
        let directory = syntax_dir.to_string();
        let syntax_dir = Path::new(&directory);
        if let Ok(mut custom_syntaxes) = SyntaxSet::load_from_folder(&syntax_dir) {
            for set in custom_syntaxes.syntaxes() {
                syntax_set.add_syntax(set.clone());
            }
            syntax_set.link_syntaxes()
        }
    };

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
