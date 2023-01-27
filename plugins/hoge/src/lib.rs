use std::sync::Mutex;
use once_cell::sync::Lazy;

static GLOBAL_DATA: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| {
    Mutex::new(Vec::new())
});

#[no_mangle]
pub unsafe extern "C" fn hoge() -> MyString {
    let mut buf = GLOBAL_DATA.lock().unwrap();
    *buf = "hello from hoge".as_bytes().to_vec();
    MyString { len: buf.len(), ptr: buf.as_ptr() }
}

#[repr(C)]
pub struct MyString {
    len: usize,
    ptr: *const u8,
}
