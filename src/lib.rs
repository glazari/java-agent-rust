extern crate jni;

use jni::sys::{JavaVM, jint};

#[no_mangle]
pub extern "C" fn Agent_OnLoad(_vm: *mut JavaVM, _options: *mut i8, _reserved: *mut i8) -> jint {
    println!("Hello from Rust Agent!");
    0 // JNI_OK
}
