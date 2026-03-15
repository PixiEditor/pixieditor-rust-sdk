use crate::abi;

pub fn load_resource(path: &str) -> Vec<u8> {
    unsafe {
        let ptr = abi::load_extension_resource(
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

