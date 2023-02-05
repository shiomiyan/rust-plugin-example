#[repr(C)]
pub struct String {
    pub len: usize,
    pub ptr: *const u8,
}