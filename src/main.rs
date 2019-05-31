use dylib::DynamicLibrary;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::{fs, io};

fn find_some_dy() -> io::Result<PathBuf> {
    let mut test_exe = std::env::current_exe()?;
    test_exe.pop();
    test_exe.push("deps");

    for entry in fs::read_dir(&test_exe)? {
        let entry = entry?;
        let name = entry.file_name().to_str().unwrap().to_string();
        if entry.path().is_file() && name.starts_with("libsome_dy") && name.ends_with(".so") {
            return Ok(entry.path());
        }
    }

    Err(io::Error::from(ErrorKind::NotFound))
}

fn main() -> Result<(), std::io::Error> {
    println!("Calling some_staticlib::get_global_direct()");
    some_staticlib::get_global_direct();


    let dy_path = find_some_dy()?;

    let lib = DynamicLibrary::open(Some(&dy_path)).expect("Cannot open dynamic library!");

    let call_from_dy = unsafe {
        match lib.symbol("get_global_var_from_dy") {
            Err(e) => {
                panic!("{:#?}", e);
            }
            Ok(f) => std::mem::transmute::<*mut u8, fn() -> u32>(f),
        }
    };
        
    let call_from_direct = unsafe {
        match lib.symbol("get_global_var_direct") {
            Err(e) => {
                panic!("{:#?}", e);
            }
            Ok(f) => std::mem::transmute::<*mut u8, fn() -> u32>(f),
        }
    };

    println!("Calling some_dy::get_global_var_from_dy)");
    call_from_dy();
    println!("Calling some_dy::cget_global_var_direct)");
    call_from_direct();

    Ok(())
}
