#![no_main]

use libfuzzer_sys::fuzz_target;
use pdf_extract::extract_text_from_mem;

fuzz_target!(|data: &[u8]| {
    let _ = extract_text_from_mem(data);
});