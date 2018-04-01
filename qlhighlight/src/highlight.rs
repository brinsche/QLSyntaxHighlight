use std::io::Cursor;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;

use core_foundation::string::CFString;
use hexplay::HexViewBuilder;
use plist::{Plist, xml::EventWriter};
use syntect::highlighting::Color;
use syntect::html::highlighted_snippet_for_string;
use syntect::parsing::SyntaxDefinition;

use util::Config;
use util::RED;

pub enum FileType {
    Binary,
    Plist,
    Syntax,
}

pub fn determine_file_type(content_type_uti: CFString) -> FileType {
    match content_type_uti.to_string().as_str() {
        "dyn.ah62d4rv4ge80c" // .a
        | "dyn.ah62d4rv4ge81g52" // .so
        | "dyn.ah62d4rv4ge81e5dmqk" // .rlib
        | "com.apple.mach-o-dylib" // .dylib
        | "com.microsoft.windows-executable" // .exe
        | "com.microsoft.windows-dynamic-link-library" // .dll
        | "com.sun.java-archive" // .jar
        | "com.sun.java-class" // .class
        | "public.executable"
        | "public.object-code" // .o
        | "public.unix-executable" => FileType::Binary,
        "com.apple.property-list" => FileType::Plist,
        _ => FileType::Syntax,
    }
}

pub fn apply_style(input: &str, conf: &Config) -> String {
    let mut buffer = String::with_capacity(input.len());
    let bg = conf.theme.settings.background.unwrap_or(Color::WHITE);
    let fg = conf.theme.settings.foreground.unwrap_or(Color::BLACK);
    let style = format!(
        "pre {{ font-size: {}px; font-family: {}; }}",
        conf.font_size, conf.font_family
    );
    buffer.push_str(&format!("<head><style>{}</style></head>", style));
    buffer.push_str(&format!(
        "<body style=\"background-color:#{:02x}{:02x}{:02x}; white-space: pre-wrap; font-size: {}px; font-family: {}; color:#{:02x}{:02x}{:02x};\">",
        bg.r, bg.g, bg.b, conf.font_size, conf.font_family, fg.r, fg.g, fg.b,
    ));
    buffer.push_str(input);
    buffer
}

pub fn syntax_highlight_file(
    buf: &Vec<u8>,
    file_path: &Path,
    conf: &Config,
) -> Result<String, ::std::io::Error> {
    let content = String::from_utf8_lossy(buf);
    let mut html = String::new();

    let first_try = panic::catch_unwind(AssertUnwindSafe(|| {
        let syntax = find_syntax_for_file(&file_path, conf);
        html = highlighted_snippet_for_string(&content, &syntax, &conf.theme);
    }));

    if first_try.is_err() {
        // Force plaintext syntax after first try panicked
        html.push_str(&format!(
            "<pre><span style=\"color:#{:02x}{:02x}{:02x}\">{}</span></pre>",
            RED.r, RED.g, RED.b, "Highlighting failed, syntax may be invalid!"
        ));

        let _retry = panic::catch_unwind(AssertUnwindSafe(|| {
            html.push_str(&highlighted_snippet_for_string(
                &content,
                &conf.syntax_set.find_syntax_plain_text(),
                &conf.theme,
            ));
        }));
    }
    Ok(apply_style(&html, conf))
}

pub fn hex_highlight_file(buf: Vec<u8>, conf: &Config) -> String {
    let view = HexViewBuilder::new(&buf)
        .codepage(::hexplay::CODEPAGE_ASCII)
        .finish();
    let result = format!("{}", view);
    apply_style(&result, conf)
}

pub fn highlight_plist(buf: &Vec<u8>, conf: &Config) -> String {
    let plist = match Plist::read(&mut Cursor::new(&buf[..])) {
        Ok(p) => p,
        Err(_) => return format_err("Error parsing .plist", conf),   
    };
    let mut plist_str = Cursor::new(Vec::new());
    {
        let mut writer = EventWriter::new(&mut plist_str);

        for item in plist.into_events() {
            match writer.write(&item) {
                Ok(_) => (),
                Err(_) => return format_err("Error parsing .plist", conf),
            };
        }
    }
    let buf = match String::from_utf8(plist_str.into_inner()) {
        Ok(s) => s,
        Err(_) => return format_err("Invalid UTF-8 in .plist", conf),
    };

    let xml_syntax = conf.syntax_set.find_syntax_by_name("XML").unwrap();
    let html = highlighted_snippet_for_string(&buf, &xml_syntax, &conf.theme);
    apply_style(&html, conf)
}

fn find_syntax_for_file<'a>(file_path: &Path, conf: &'a Config) -> &'a SyntaxDefinition {
    match conf.syntax_set.find_syntax_for_file(&file_path) {
        Ok(found) => match found {
            Some(syntax) => syntax,
            None => file_path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .and_then(|filename| conf.syntax_set.find_syntax_by_token(filename))
                .unwrap_or_else(|| conf.syntax_set.find_syntax_plain_text()),
        },
        Err(_) => conf.syntax_set.find_syntax_plain_text(),
    }
}

pub fn format_err(cause: &str, conf: &Config) -> String {
    let mut error = String::new();
    error.push_str(&format!(
        "<pre><span style=\"color:#{:02x}{:02x}{:02x}\">{}</span></pre>",
        RED.r, RED.g, RED.b, cause
    ));
    apply_style(&error, &conf)
}
