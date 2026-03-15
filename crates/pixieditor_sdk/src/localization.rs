use std::ffi::CStr;
use crate::abi;

pub fn translate(key: &str) -> String {
    unsafe {
        let ptr = abi::translate_key(key.as_ptr(), key.len() as i32);
        if ptr.is_null() {
            return String::new();
        }

        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

