use std::sync::Mutex;
use once_cell::sync::Lazy;
use types;

static GLOBAL_DATA: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| {
    Mutex::new(Vec::new())
});

#[no_mangle]
pub unsafe extern "C" fn fuga() -> types::String {
    let mut buf = GLOBAL_DATA.lock().unwrap();
    *buf = "hello from fuga".as_bytes().to_vec();
    types::String { len: buf.len(), ptr: buf.as_ptr() }
}