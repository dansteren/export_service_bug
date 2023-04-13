#[candid::candid_method(query)]
#[ic_cdk_macros::query(name = "candid_method")]
pub fn rust_method() -> bool {
    true
}

candid::export_service!();

#[no_mangle]
pub fn get_candid_pointer() -> *mut std::os::raw::c_char {
    std::ffi::CString::new(__export_service())
        .unwrap()
        .into_raw()
}
