use crate::abi;

pub fn get_installed_extensions() -> Vec<String> {
    unsafe {
        let ptr = unsafe { abi::api_get_installed_extensions() };

        if ptr.is_null() {
            return vec![];
        }

        let mut offset = 0usize;

        let total_size = crate::byte_utils::read_u32(ptr, &mut offset) as usize;
        if total_size == 0 {
            return vec![];
        }

        let data_ptr = ptr.add(offset);

        let mut inner_offset = 0usize;

        let count = crate::byte_utils::read_u32(data_ptr, &mut inner_offset) as usize;

        let mut result = Vec::with_capacity(count);

        for _ in 0..count {
            let len = crate::byte_utils::read_u32(data_ptr, &mut inner_offset) as usize;

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