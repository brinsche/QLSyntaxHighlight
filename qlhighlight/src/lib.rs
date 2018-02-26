use std::ffi::CString;

#[no_mangle]
pub extern "C" fn highlight_html() -> CString {
    let html = "<head><title>/Users/me/Desktop/hello/src/main.rs</title><style>\n pre {\n font-size:13px;\n font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;\n }</style></head>\n<body style=\"background-color:#2b303b;\">\n<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {</span>\n<span style=\"color:#c0c5ce;\">    println!(&quot;</span><span style=\"color:#a3be8c;\">Hello, world!</span><span style=\"color:#c0c5ce;\">&quot;);</span>\n<span style=\"color:#c0c5ce;\">}</span>\n</pre>\n</body>";
    CString::new(html).unwrap()
}
