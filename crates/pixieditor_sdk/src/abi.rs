use std::os::raw::{c_char, c_double};

type WindowHandle = i32;

#[link(wasm_import_module = "env")]
unsafe extern "C" {

    #[link_name = "set_window_title"]
    pub fn set_window_title(
        window_handle: WindowHandle,
        title: *const c_char,
        title_len: i32,
    );

    #[link_name = "get_window_title"]
    pub fn get_window_title(window_handle: WindowHandle) -> *const c_char;

    #[link_name = "get_window_width"]
    pub fn get_window_width(window_handle: WindowHandle) -> c_double;

    #[link_name = "set_window_width"]
    pub fn set_window_width(window_handle: WindowHandle, width: c_double);

    #[link_name = "get_window_height"]
    pub fn get_window_height(window_handle: WindowHandle) -> c_double;

    #[link_name = "set_window_height"]
    pub fn set_window_height(window_handle: WindowHandle, height: c_double);

    #[link_name = "show_window"]
    pub fn show_window(window_handle: WindowHandle);

    #[link_name = "close_window"]
    pub fn close_window(window_handle: WindowHandle);

    #[link_name = "get_window"]
    pub fn get_window(id_ptr: *const c_char, id_len: i32) -> WindowHandle;

    #[link_name = "log_message"]
    pub fn log_message(ptr: *const u8, len: i32);

    #[link_name = "invoke_command_null_param"]
    pub fn invoke_command_null_param(
        name_ptr: *const u8,
        name_len: i32,
    );

    #[link_name = "invoke_command_string"]
    pub fn invoke_command_string(
        name_ptr: *const u8,
        name_len: i32,
        param_ptr: *const u8,
        param_len: i32,
    );

    #[link_name = "invoke_command_int"]
    pub fn invoke_command_int(
        name_ptr: *const u8,
        name_len: i32,
        param: i32,
    );

    #[link_name = "invoke_command_bool"]
    pub fn invoke_command_bool(
        name_ptr: *const u8,
        name_len: i32,
        param: bool,
    );

    #[link_name = "invoke_command_float"]
    pub fn invoke_command_float(
        name_ptr: *const u8,
        name_len: i32,
        param: f32,
    );

    #[link_name = "invoke_command_double"]
    pub fn invoke_command_double(
        name_ptr: *const u8,
        name_len: i32,
        param: f64,
    );

    #[link_name = "invoke_command_bytes"]
    pub fn invoke_command_bytes(
        name_ptr: *const u8,
        name_len: i32,
        ptr: *const u8,
        length: i32,
    );

    #[link_name = "register_brush_tool"]
    pub fn register_brush_tool(
        pixi_file_ptr: *const u8,
        pixi_file_len: i32,
        config_file_ptr: *const u8,
        config_file_len: i32,
    );

    #[link_name = "add_tool_to_toolset"]
    pub fn add_tool_to_toolset(
        tool_name_ptr: *const u8,
        tool_name_len: i32,
        toolset_name_ptr: *const u8,
        toolset_name_len: i32,
        at_index: i32,
    );

    #[link_name = "load_extension_resource"]
    pub fn load_extension_resource(
        path_ptr: *const u8,
        path_len: i32) -> *const u8;

    #[link_name = "translate_key"]
    pub fn translate_key(key: *const u8, key_len: i32) -> *const c_char;

    #[link_name = "get_owned_content"]
    pub fn api_get_owned_content() -> *const u8;

    #[link_name = "is_user_logged_in"]
    pub fn api_is_user_logged_in() -> i32;

    #[link_name = "get_username"]
    pub fn api_get_username(ptr_out: *mut *const u8, len_out: *mut i32);

    #[link_name = "get_account_provider_name"]
    pub fn api_get_account_provider_name(ptr_out: *mut *const u8, len_out: *mut i32);
}
