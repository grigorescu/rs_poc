use std::str;

// Generally speaking, our strings might have NULs, so we use this (pointer, length) struct
#[derive(Debug)]
#[repr(C)]
pub struct StringN {
	string: *const u8,
	length: usize,
}

impl From<&str> for StringN {
	fn from(s: &str) -> Self {
		Self { string: s.as_ptr(), length: s.len() }
	}
}

impl From<&[u8]> for StringN {
	fn from(s: &[u8]) -> Self {
		Self { string: s.as_ptr(), length: s.len() }
	}
}
