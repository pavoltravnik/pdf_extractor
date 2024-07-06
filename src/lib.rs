use pdf_extract::extract_text_from_mem;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn extract_text_from_pdf(path_pdf_clone: *const c_char, path_txt: *const c_char) -> *mut c_char {
    let c_str_path_pdf_clone = unsafe { CStr::from_ptr(path_pdf_clone) };
    let c_str_path_txt = unsafe { CStr::from_ptr(path_txt) };

    let path_pdf_clone_str = match c_str_path_pdf_clone.to_str() {
        Ok(str) => str,
        Err(_) => return CString::new("Invalid PDF path").unwrap().into_raw(),
    };
    let path_txt_str = match c_str_path_txt.to_str() {
        Ok(str) => str,
        Err(_) => return CString::new("Invalid TXT path").unwrap().into_raw(),
    };

    if Path::new(&path_pdf_clone_str).exists() && !Path::new(&path_txt_str).exists() {
        if let Some(parent) = Path::new(&path_txt_str).parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return CString::new(format!("Failed to create directories: {}", e)).unwrap().into_raw();
            }
        }
        let bytes = match fs::read(&path_pdf_clone_str) {
            Ok(b) => b,
            Err(e) => return CString::new(format!("Failed to read PDF: {}", e)).unwrap().into_raw(),
        };
        let text = match extract_text_from_mem(&bytes) {
            Ok(t) => t,
            Err(e) => return CString::new(format!("Failed to extract text: {}", e)).unwrap().into_raw(),
        };

        let re_double_newlines = Regex::new(r"\n{2,}").unwrap();
        let intermediate = re_double_newlines.replace_all(&text, "@@@");

        let re_single_newline = Regex::new(r"\n").unwrap();
        let with_spaces = re_single_newline.replace_all(&intermediate, " ");

        let with_single_newline = with_spaces.replace("@@@", "\n");

        let re_double_spaces = Regex::new(r" {2,}").unwrap();
        let result = re_double_spaces.replace_all(&with_single_newline, " ").to_string();

        if let Err(e) = fs::write(path_txt_str, &result) {
            return CString::new(format!("Failed to write TXT: {}", e)).unwrap().into_raw();
        }
        return CString::new("txt saved").unwrap().into_raw();
    } else {
        return CString::new("Skipping txt conversion").unwrap().into_raw();
    }
}
