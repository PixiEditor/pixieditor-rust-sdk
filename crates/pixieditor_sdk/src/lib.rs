mod abi;
mod commands;
mod brushes;
mod localization;
mod resources;
mod logging;
mod windowing;
mod extension;
mod pixieditor_extension;
mod ProtoAutogen;

pub use brushes::*;
pub use localization::*;
pub use resources::*;
pub use windowing::*;
pub use extension::*;
pub use crate::ProtoAutogen::protos::*;
pub use prost::Message;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated_crypto.rs"));

#[unsafe(no_mangle)]
pub extern "C" fn get_encryption_key() -> *const u8 {
    KEY.as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_encryption_iv() -> *const u8 {
    IV.as_ptr()
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