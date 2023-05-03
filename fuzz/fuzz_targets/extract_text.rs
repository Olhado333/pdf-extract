#![no_main]

use std::fs;
use std::path::Path;
use libfuzzer_sys::fuzz_target;
use pdf_extract::extract_text;

fuzz_target!(|data: &[u8]| {
    let path = Path::new("test.pdf");
    let pdf = String::from_utf8_lossy(data).to_string();

    match fs::write(path, pdf) {
        Ok(_) => (),
        Err(_) => return,
    }

    let _ = extract_text(path);
});