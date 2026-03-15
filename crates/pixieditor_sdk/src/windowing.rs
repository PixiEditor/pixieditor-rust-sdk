use std::ffi::{CStr, CString};
use crate::{abi};
use crate::abi::{close_window, get_window_title, set_window_title, show_window};

type WindowHandle = i32;

pub fn window_from_id(id: &str) -> WindowHandle {
    let c = CString::new(id).unwrap();
    unsafe { abi::get_window(c.as_ptr(), id.len() as i32) }
}

pub fn set_title(window: WindowHandle, title: &str) {
    let c = CString::new(title).unwrap();
    unsafe {
        abi::set_window_title(window, c.as_ptr(), title.len() as i32);
    }
}

pub fn get_title(window: WindowHandle) -> String {
    unsafe {
        let ptr = abi::get_window_title(window);
        if ptr.is_null() {
            return String::new();
        }

        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn show(window: WindowHandle) {
    unsafe { abi::show_window(window) }
}

pub fn close(window: WindowHandle) {
    unsafe { abi::close_window(window) }
}
