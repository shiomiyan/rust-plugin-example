use types;

pub fn call_hoge_dynamic() -> Result<types::String, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("target/debug/libhoge.so")?;
        let func: libloading::Symbol<unsafe extern fn() -> types::String> = lib.get(b"hoge")?;
        Ok(func())
    }
}

pub fn call_fuga_dynamic() -> Result<types::String, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("target/debug/libfuga.so")?;
        let func: libloading::Symbol<unsafe extern fn() -> types::String> = lib.get(b"fuga")?;
        Ok(func())
    }
}