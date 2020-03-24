use ffi_support::FfiStr;
use std::os::raw::{c_char};
use std::ffi::CString;


/// [`FfiStr`], from the `ffi-support` crate, makes passing C-style (null-terminated) strings a little more convenient
/// and a little safer. The `to` and `name` variables here cannot outlive the call to `rust_hello`.
#[no_mangle]
pub extern fn rust_hello(to: FfiStr) -> *mut c_char {
    let name = to.as_opt_str().unwrap_or("there");
    let reply = format!("Hello {}", name);
    CString::new(reply)
        .expect("string without a nul byte")
        .into_raw()
}

// Strings passed from Rust back to Swift should be freed when they are no longer needed. This macro exposes a
// safe function to Swift that allows it do so.
ffi_support::define_string_destructor!(free_rust_string);

#[no_mangle]
pub extern fn use_swift_closure(swift_fn: extern "C" fn(i32) -> i32) {
    let input : i32 = 10;
    let output = swift_fn(input);
    println!("Swift closure turned {:?} into {:?}", input, output);
}
