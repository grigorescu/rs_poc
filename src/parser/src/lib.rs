mod common;
mod parser;

use std::os::raw::c_int;
use std::slice;

use common::*;
use parser::*;

// This is what we return back to C++.
#[repr(C)]
pub struct SyslogFields {
	success: bool,
	error: StringN,
	priority: Priority,
	msg: StringN,
}


// This is the function that our C++ code calls.
#[no_mangle]
pub unsafe extern "C" fn parse(len: c_int, raw_data: *const u8, _orig: bool,
	                           _seq: u64, _ip: *const libc::c_void, _caplen: c_int) -> SyslogFields {

	if raw_data.is_null()
		{
		let msg = "";
		let error = "Null pointer received as packet data.";
		return SyslogFields { success: false, error: StringN::from(error),
	                          priority: Priority {facility: 0, severity: 0}, msg: StringN::from(msg) };
		}

	let data: &[u8] = slice::from_raw_parts(raw_data, len as usize);
	let (_, result) = SyslogMessage::parse(data).expect("Could not parse as syslog");

	let error = "";
	SyslogFields { success: true, error: StringN::from(error),
	               priority: result.pri, msg: common::StringN::from(data) }
}
