#![deny(improper_ctypes, improper_ctypes_definitions)]

use anyhow_serde::anyhow;

#[no_mangle]
pub extern "C" fn anyhow1(err: anyhow_serde::Error) {
    println!("{:?}", err);
}

#[no_mangle]
pub extern "C" fn anyhow2(err: &mut Option<anyhow_serde::Error>) {
    *err = Some(anyhow!("ffi error"));
}

#[no_mangle]
pub extern "C" fn anyhow3() -> Option<anyhow_serde::Error> {
    Some(anyhow!("ffi error"))
}
