use std::io::prelude::*;
use std::fs::File;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;

use syntect::highlighting::Color;
use syntect::html::highlighted_snippet_for_string;

use util::Config;

pub fn apply_style(input: &str, conf: &Config) -> String {
    let mut buffer = String::new();

    let bg = conf.theme.settings.background.unwrap_or(Color::WHITE);
    buffer.push_str(&format!(
        "<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
        bg.r, bg.g, bg.b
    ));
    let style = format!(
        "pre {{ font-size: {}px; font-family: {}; }}",
        conf.font_size, conf.font_family
    );
    buffer.push_str(&format!("<head><style>{}</style></head>", style));

    buffer.push_str(input);
    buffer
}

pub fn highlight_file(file_path: &Path, conf: &Config) -> Result<String, ::std::io::Error> {
    let mut filecontent: Vec<u8> = Vec::new();
    let mut file = File::open(&file_path)?;
    file.read_to_end(&mut filecontent)?;

    let content = String::from_utf8_lossy(&filecontent);
    let mut html = String::new();

    let first_try = panic::catch_unwind(AssertUnwindSafe(|| {
        let syntax = match conf.syntax_set.find_syntax_for_file(&file_path) {
            Ok(found) => match found {
                Some(syntax) => syntax,
                None => file_path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .and_then(|filename| conf.syntax_set.find_syntax_by_token(filename))
                    .unwrap_or_else(|| conf.syntax_set.find_syntax_plain_text()),
            },
            Err(_) => conf.syntax_set.find_syntax_plain_text(),
        };

        html = highlighted_snippet_for_string(&content, &syntax, &conf.theme);
    }));

    if first_try.is_err() {
        // Force plaintext syntax after first try panicked
        let c = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };

        html.push_str(&format!(
            "<pre><span style=\"color:#{:02x}{:02x}{:02x}\">{}</span></pre>\n",
            c.r, c.g, c.b, "Highlighting failed, syntax may be invalid!"
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
