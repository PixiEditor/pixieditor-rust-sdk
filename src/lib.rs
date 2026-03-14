use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::{c_char, c_double, c_int};
type WindowHandle = i32;

pub mod ProtoAutogen;
use prost::Message;

use crate::ProtoAutogen::protos::CustomToolConfig;
use crate::ProtoAutogen::protos::Shortcut;
use crate::ProtoAutogen::protos::ActionDisplayConfig;

//
// IMPORTS (host functions)
//

#[link(wasm_import_module = "env")]
unsafe extern "C" {

    #[link_name = "set_window_title"]
    fn set_window_title(
        window_handle: WindowHandle,
        title: *const c_char,
        title_len: i32,
    );

    #[link_name = "get_window_title"]
    fn get_window_title(window_handle: WindowHandle) -> *const c_char;

    #[link_name = "get_window_width"]
    fn get_window_width(window_handle: WindowHandle) -> c_double;

    #[link_name = "set_window_width"]
    fn set_window_width(window_handle: WindowHandle, width: c_double);

    #[link_name = "get_window_height"]
    fn get_window_height(window_handle: WindowHandle) -> c_double;

    #[link_name = "set_window_height"]
    fn set_window_height(window_handle: WindowHandle, height: c_double);

    #[link_name = "show_window"]
    fn show_window(window_handle: WindowHandle);

    #[link_name = "close_window"]
    fn close_window(window_handle: WindowHandle);

    #[link_name = "get_window"]
    fn get_window(id_ptr: *const c_char, id_len: i32) -> WindowHandle;

   #[link_name = "log_message"]
    fn log_message(ptr: *const u8, len: i32);

 #[link_name = "invoke_command_null_param"]
    fn invoke_command_null_param(
        name_ptr: *const u8,
        name_len: i32,
    );

    #[link_name = "invoke_command_string"]
    fn invoke_command_string(
        name_ptr: *const u8,
        name_len: i32,
        param_ptr: *const u8,
        param_len: i32,
    );

    #[link_name = "invoke_command_int"]
    fn invoke_command_int(
        name_ptr: *const u8,
        name_len: i32,
        param: i32,
    );

    #[link_name = "invoke_command_bool"]
    fn invoke_command_bool(
        name_ptr: *const u8,
        name_len: i32,
        param: bool,
    );

    #[link_name = "invoke_command_float"]
    fn invoke_command_float(
        name_ptr: *const u8,
        name_len: i32,
        param: f32,
    );

    #[link_name = "invoke_command_double"]
    fn invoke_command_double(
        name_ptr: *const u8,
        name_len: i32,
        param: f64,
    );

    #[link_name = "invoke_command_bytes"]
    fn invoke_command_bytes(
        name_ptr: *const u8,
        name_len: i32,
        ptr: *const u8,
        length: i32,
    );

    #[link_name = "register_brush_tool"]
    fn register_brush_tool(
        pixi_file_ptr: *const u8,
        pixi_file_len: i32,
        config_file_ptr: *const u8,
        config_file_len: i32,
    );

    #[link_name = "add_tool_to_toolset"]
    fn add_tool_to_toolset(
        tool_name_ptr: *const u8,
        tool_name_len: i32,
        toolset_name_ptr: *const u8,
        toolset_name_len: i32,
        at_index: i32,
    );

    #[link_name = "load_extension_resource"]
    fn load_extension_resource(
        path_ptr: *const u8,
        path_len: i32) -> *const u8;

    #[link_name = "translate_key"]
    fn translate_key(key: *const u8, key_len: i32) -> *const c_char;
}

//
// SAFE WRAPPERS
//

#[unsafe(no_mangle)]
pub extern "C" fn command_invoked_str_param(
    name_ptr: *const u8,
    name_len: i32,
    param_ptr: *const u8,
    param_len: i32,
) {}

pub fn invoke(name: &str) {
    unsafe {
        invoke_command_null_param(name.as_ptr(), name.len() as i32);
    }
}

pub fn invoke_str(name: &str, param: &str) {
    unsafe {
        invoke_command_string(
            name.as_ptr(),
            name.len() as i32,
            param.as_ptr(),
            param.len() as i32,
        );
    }
}

pub fn invoke_int(name: &str, param: i32) {
    unsafe {
        invoke_command_int(name.as_ptr(), name.len() as i32, param);
    }
}

pub fn invoke_bool(name: &str, param: bool) {
    unsafe {
        invoke_command_bool(name.as_ptr(), name.len() as i32, param);
    }
}

pub fn invoke_float(name: &str, param: f32) {
    unsafe {
        invoke_command_float(name.as_ptr(), name.len() as i32, param);
    }
}

pub fn invoke_double(name: &str, param: f64) {
    unsafe {
        invoke_command_double(name.as_ptr(), name.len() as i32, param);
    }
}

pub fn invoke_bytes(name: &str, data: &[u8]) {
    unsafe {
        invoke_command_bytes(
            name.as_ptr(),
            name.len() as i32,
            data.as_ptr(),
            data.len() as i32,
        );
    }
}

pub fn register_brush(pixi_file: &[u8], config_file: &[u8]) {
    unsafe {
        register_brush_tool(
            pixi_file.as_ptr(),
            pixi_file.len() as i32,
            config_file.as_ptr(),
            config_file.len() as i32,
        );
    }
}

pub fn add_to_toolset(tool_name: &str, toolset_name: &str, at_index: i32) {
    unsafe {
        add_tool_to_toolset(
            tool_name.as_ptr(),
            tool_name.len() as i32,
            toolset_name.as_ptr(),
            toolset_name.len() as i32,
            at_index,
        );
    }
}

pub fn translate(key: &str) -> String {
    unsafe {
        let ptr = translate_key(key.as_ptr(), key.len() as i32);
        if ptr.is_null() {
            return String::new();
        }

        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn load_resource(path: &str) -> Vec<u8> {
    unsafe {
        let ptr = load_extension_resource(
            path.as_ptr(),
            path.len() as i32,
        );

        if ptr.is_null() {
            return Vec::new();
        }

        let len = *(ptr as *const i32) as usize;

        if len == 0 {
            return Vec::new();
        }

        let data_ptr = ptr.add(std::mem::size_of::<i32>());
        std::slice::from_raw_parts(data_ptr, len).to_vec()
    }
}

pub fn log(message: &str) {
    unsafe {
        log_message(message.as_ptr(), message.len() as i32);
    }
}

pub fn window_from_id(id: &str) -> WindowHandle {
    let c = CString::new(id).unwrap();
    unsafe { get_window(c.as_ptr(), id.len() as i32) }
}

pub fn set_title(window: WindowHandle, title: &str) {
    let c = CString::new(title).unwrap();
    unsafe {
        set_window_title(window, c.as_ptr(), title.len() as i32);
    }
}

pub fn get_title(window: WindowHandle) -> String {
    unsafe {
        let ptr = get_window_title(window);
        if ptr.is_null() {
            return String::new();
        }

        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn show(window: WindowHandle) {
    unsafe { show_window(window) }
}

pub fn close(window: WindowHandle) {
    unsafe { close_window(window) }
}

//
// EXPORTS (callbacks invoked by host)
//

#[unsafe(no_mangle)]
pub extern "C" fn load() {
    // extension load entry point
}

#[unsafe(no_mangle)]
pub extern "C" fn initialize() {

}

static KEY: [u8; 16] = [0; 16];
static IV: [u8; 16] = [0; 16];

#[unsafe(no_mangle)]
pub extern "C" fn get_encryption_key() -> *const u8 {
    KEY.as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_encryption_iv() -> *const u8 {
    IV.as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn command_invoked(name: *const c_char) {
    if name.is_null() {
        return;
    }

    let name = unsafe { CStr::from_ptr(name) }
        .to_string_lossy();
}

#[unsafe(no_mangle)]
pub extern "C" fn user_ready() {
}

#[unsafe(no_mangle)]
pub extern "C" fn main_window_loaded() {
}

#[unsafe(no_mangle)]
pub extern "C" fn on_user_logged_in() {
}

#[unsafe(no_mangle)]
pub extern "C" fn on_user_logged_out() {
}

#[unsafe(no_mangle)]
pub extern "C" fn _malloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn _free(ptr: *mut u8){
    if ptr.is_null() {
        return;
    }

    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, 0);
    }
}