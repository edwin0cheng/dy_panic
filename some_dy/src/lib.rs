#[no_mangle]
pub extern "C" fn get_global_var_from_dy() -> u32 {
    some_staticlib::get_global_from_dy()
}

#[no_mangle]
pub extern "C" fn get_global_var_direct() -> u32 {
    some_staticlib::get_global_direct()
}
