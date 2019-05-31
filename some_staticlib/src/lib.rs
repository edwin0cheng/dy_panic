static GLOBAL_VAR: u32 = 1;

pub fn get_global_direct() -> u32 {
    let p_global_var = &GLOBAL_VAR as *const _;
    dbg!(p_global_var);
    return GLOBAL_VAR;
}

pub fn get_global_from_dy() -> u32 {
    let p_global_var = &GLOBAL_VAR as *const _;
    dbg!(p_global_var);
    return GLOBAL_VAR;
}


