This example project demonstrates the dynamic linkage behavior of override static variable in some situation:

## How to run
```
cargo run
```

## Output (In Ubuntu 16.04)
```
Calling some_staticlib::get_global_direct()
[some_staticlib/src/lib.rs:5] p_global_var = 0x00007f977c43695c
Calling some_dy::get_global_var_from_dy)
[some_staticlib/src/lib.rs:11] p_global_var = 0x00007f977b1c16c0
Calling some_dy::cget_global_var_direct)
[some_staticlib/src/lib.rs:5] p_global_var = 0x00007f977c43695c
```
## The "meat" of the code

https://github.com/edwin0cheng/dy_panic/blob/98e399a70d26eb754cc09d320090d098ac25980c/src/main.rs#L22-L55

https://github.com/edwin0cheng/dy_panic/blob/98e399a70d26eb754cc09d320090d098ac25980c/some_dy/src/lib.rs#L1-L9

https://github.com/edwin0cheng/dy_panic/blob/98e399a70d26eb754cc09d320090d098ac25980c/some_staticlib/src/lib.rs#L1-L15

## Explanation

In src/main.rs:

* call `some_staticlib::get_global_direct()`  (static linkage)
* call `some_dy::get_global_var_from_dy` (dynamic linkgage)
  * in some_dy , call `some_staticlib::get_global_var_from_dy()` (static linkage)
* call `some_dy::get_global_var_direct` (dynamic linkgage)
  * in some_dy , call `some_staticlib::get_global_var_direct()` (static linkage)
  
Note that although `some_dy::get_global_var_from_dy` and `some_dy::get_global_var_direct` are wrapped in dynamic library, the dynamic linker is smart enought to know the underneath `some_staticlib::get_global_var_direct()` can be shared.
  
