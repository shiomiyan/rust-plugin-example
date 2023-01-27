pub fn call_hoge_dynamic() -> Result<MyString, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("target/debug/libhoge.so")?;
        let func: libloading::Symbol<unsafe extern fn() -> MyString> = lib.get(b"hoge")?;
        Ok(func())
    }
}

pub fn call_fuga_dynamic() -> Result<MyString, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("target/debug/libfuga.so")?;
        let func: libloading::Symbol<unsafe extern fn() -> MyString> = lib.get(b"fuga")?;
        Ok(func())
    }
}

#[repr(C)]
pub struct MyString {
    pub len: usize,
    pub ptr: *const u8,
}
