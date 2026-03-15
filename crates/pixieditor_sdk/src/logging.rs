use crate::abi;

pub fn log(message: &str) {
    unsafe {
        abi::log_message(message.as_ptr(), message.len() as i32);
    }
}

