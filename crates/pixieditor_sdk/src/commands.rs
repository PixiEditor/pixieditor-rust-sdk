use crate::abi;
pub fn invoke_command(name: &str) {
    unsafe {
        abi::invoke_command_null_param(name.as_ptr(), name.len() as i32);
    }
}

pub fn invoke_command_str(name: &str, param: &str) {
    unsafe {
        abi::invoke_command_string(
            name.as_ptr(),
            name.len() as i32,
            param.as_ptr(),
            param.len() as i32,
        );
    }
}

pub fn invoke_command_int(name: &str, param: i32) {
    unsafe {
        abi::invoke_command_int(name.as_ptr(), name.len() as i32, param);
    }
}

pub fn invoke_command_bool(name: &str, param: bool) {
    unsafe {
        abi::invoke_command_bool(name.as_ptr(), name.len() as i32, param);
    }
}

pub fn invoke_command_float(name: &str, param: f32) {
    unsafe {
        abi::invoke_command_float(name.as_ptr(), name.len() as i32, param);
    }
}

pub fn invoke_command_double(name: &str, param: f64) {
    unsafe {
        abi::invoke_command_double(name.as_ptr(), name.len() as i32, param);
    }
}

pub fn invoke_command_bytes(name: &str, data: &[u8]) {
    unsafe {
        abi::invoke_command_bytes(
            name.as_ptr(),
            name.len() as i32,
            data.as_ptr(),
            data.len() as i32,
        );
    }
}
