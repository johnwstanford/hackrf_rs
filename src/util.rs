
use std::ffi::CStr;

use libc::c_char;

pub unsafe fn cstr_ptr_to_string(ptr:*const c_char) -> String {

	let cstr = CStr::from_ptr(ptr);
	let rstr = cstr.to_str().unwrap();
	rstr.to_owned()

}