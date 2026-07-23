pub(crate) fn read_u32(base: *const u8, offset: &mut usize) -> u32 {
    unsafe {
        let ptr = base.add(*offset) as *const u32;
        let value = ptr.read_unaligned();
        *offset += 4;
        u32::from_le(value)
    }
}
