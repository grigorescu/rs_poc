extern crate libc;
extern crate nom;

use crate::common::*;

use nom::IResult;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::combinator::rest;

use std::str;

#[derive(Debug)]
pub struct SyslogMessage {
	pub pri: Priority,
	pub msg: StringN,
}


impl SyslogMessage {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		let (input, priority) = Priority::parse(input)?;
		let (input, message) = rest(input)?;
		// Our message is the rest of the input
		Ok((input, Self { pri: priority, msg: StringN::from(message) }))
	}
}

#[derive(Debug)]
#[repr(C)]
pub struct Priority {
  pub facility: u8,
  pub severity: u8,
}

impl Priority {
	pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
		// Format is <PRI>
		let (input, _) = char('<')(input)?;
		let (input, priority) = is_not(">")(input)?;
		let (input, _) = char('>')(input)?;

		let priority: u16 = str::from_utf8(priority).expect("Could not convert to string; possible UTF-8 encoding error")
		                                            .parse().expect("Could not parse priority as an integer");

		let facility: u8 = (priority / 8) as u8;
		let severity: u8 = (priority % 8) as u8;

		Ok((input, Self { facility: facility, severity: severity }))
	}
}


#[test]
fn test_priority_facility() {
	assert_eq!(Priority::parse(b"<0>").unwrap().1.facility, 0);
	assert_eq!(Priority::parse(b"<00>").unwrap().1.facility, 0);
	assert_eq!(Priority::parse(b"<000>").unwrap().1.severity, 0);

	assert_eq!(Priority::parse(b"<8>").unwrap().1.facility, 1);
	assert_eq!(Priority::parse(b"<184>").unwrap().1.facility, 23);
}

#[test]
fn test_priority_severity() {
	assert_eq!(Priority::parse(b"<0>").unwrap().1.severity, 0);
	assert_eq!(Priority::parse(b"<00>").unwrap().1.severity, 0);
	assert_eq!(Priority::parse(b"<000>").unwrap().1.severity, 0);

	assert_eq!(Priority::parse(b"<7>").unwrap().1.severity, 7);
	assert_eq!(Priority::parse(b"<191>").unwrap().1.severity, 7);
}
