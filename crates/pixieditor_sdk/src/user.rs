use crate::abi::{api_get_account_provider_name, api_get_owned_content, api_get_username, api_is_user_logged_in, log_message};
use crate::log;

pub fn is_logged_in() -> bool {
    unsafe { api_is_user_logged_in() != 0 }
}

fn read_host_string(f: unsafe extern "C" fn(*mut *const u8, *mut i32)) -> String {
    unsafe {
        let mut ptr: *const u8 = std::ptr::null();
        let mut len: i32 = 0;

        f(&mut ptr, &mut len);

        if ptr.is_null() || len == 0 {
            return String::new();
        }

        let slice = std::slice::from_raw_parts(ptr, len as usize);
        std::str::from_utf8(slice).unwrap().to_owned()
    }
}

pub fn get_username() -> String {
    read_host_string(api_get_username)
}

pub fn get_account_provider_name() -> String {
    read_host_string(api_get_account_provider_name)
}

pub fn get_owned_content() -> Vec<String> {
    unsafe {
        let ptr = api_get_owned_content();
        if ptr.is_null() {
            return vec![];
        }

        let mut offset = 0usize;

        let total_size = read_u32(ptr, &mut offset) as usize;
        if total_size == 0 {
            return vec![];
        }

        let data_ptr = ptr.add(offset);

        let mut inner_offset = 0usize;

        let count = read_u32(data_ptr, &mut inner_offset) as usize;

        let mut result = Vec::with_capacity(count);

        for _ in 0..count {
            let len = read_u32(data_ptr, &mut inner_offset) as usize;

            let slice = std::slice::from_raw_parts(
                data_ptr.add(inner_offset),
                len,
            );

            let s = std::str::from_utf8(slice)
                .unwrap_or("<invalid utf8>")
                .to_string();

            result.push(s);

            inner_offset += len;
        }

        result
    }
}

fn read_u32(base: *const u8, offset: &mut usize) -> u32 {
    unsafe {
        let ptr = base.add(*offset) as *const u32;
        let value = ptr.read_unaligned();
        *offset += 4;
        u32::from_le(value)
    }
}