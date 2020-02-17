extern crate libc;


use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use libc::c_char;


pub(crate) extern "C" fn convert_to_utf8<'a>(input_string: *const c_char) -> &'a str {
	let string_pointer = unsafe {
		CStr::from_ptr(input_string)
	};
	string_pointer.to_str().unwrap()
}


pub(crate) extern "C" fn convert_to_utf16<'a>(input_string: &str) -> *const c_char {
	CString::new(input_string.as_bytes()).unwrap().into_raw()
}