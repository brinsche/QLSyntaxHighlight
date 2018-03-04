use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFType, TCFType};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::dictionary::{CFDictionary, CFDictionaryRef};

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

#[derive(Debug)]
pub struct Config {
    pub font_size: CFString,
    pub font_family: CFString,
    pub theme_name: CFString,
}

pub fn get_settings() -> Config {
    let keys = CFArray::from_CFTypes(&[
        CFString::new("fontSize"),
        CFString::new("fontFamily"),
        CFString::new("theme"),
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
        .find2(&CFString::new("fontSize"))
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(ptr).downcast::<CFString>() })
        .unwrap_or(CFString::new("11"));

    let font_family = prefs
        .find2(&CFString::new("fontFamily"))
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(ptr).downcast::<CFString>() })
        .unwrap_or(CFString::new("Menlo, monospace"));

    let theme_name = prefs
        .find2(&CFString::new("theme"))
        .and_then(|ptr| unsafe { CFType::wrap_under_create_rule(ptr).downcast::<CFString>() })
        .unwrap_or(CFString::new("InspiredGithub"));

    Config {
        font_size,
        font_family,
        theme_name,
    }
}
