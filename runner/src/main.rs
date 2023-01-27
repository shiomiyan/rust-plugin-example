use plugin_loader;

fn main() {
    let hoge_lib = plugin_loader::call_hoge_dynamic().unwrap();
    let hoge = unsafe {
        std::slice::from_raw_parts(hoge_lib.ptr, hoge_lib.len)
    };
    let hoge_s = String::from_utf8(hoge.to_owned()).unwrap();
    println!("{:?}", hoge_s);

    let fuga_lib = plugin_loader::call_hoge_dynamic().unwrap();
    let fuga = unsafe {
        std::slice::from_raw_parts(fuga_lib.ptr, fuga_lib.len)
    };
    let fuga_s = String::from_utf8(fuga.to_owned()).unwrap();
    println!("{:?}", fuga_s);
}