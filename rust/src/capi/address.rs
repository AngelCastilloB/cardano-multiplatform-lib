
use super::*;

#[no_mangle]
pub unsafe extern fn get_float() -> libc::c_float {
    0.1
}