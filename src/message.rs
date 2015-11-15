
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::str;

/// A generic http message that can be either a request or response
pub struct Message<'a> {
	/// This will be parsed by the respective message type (request or response)
	initial_line: &'a str,
	headers: HashMap<&'a str, &'a str>,
}

#[derive(Debug)]
pub enum ParseError {
	InvalidLineEndings,
	InvalidHeaders,
	InvalidEncoding(str::Utf8Error),
}

struct LineEndingFormat<'a> {
	line_separator: &'a str,
	headers_and_body_separator: &'a str
}

const CRLF : LineEndingFormat<'static> =
    LineEndingFormat {line_separator: "\r\n", headers_and_body_separator: "\r\n\r\n"};

const LF : LineEndingFormat<'static> =
    LineEndingFormat {line_separator: "\n", headers_and_body_separator: "\r\n"};
    
impl<'a> Message<'a> {

	/// Messages may end in CRLF (\r\n) or LF (\n).
	/// We find the first \n which should always be there, then check if a \r is behind that \n.
	fn get_line_endings(message_text: &'a str) -> Result<LineEndingFormat, ParseError> {
		let expected_cr_position = try!(message_text.find('\n').ok_or(ParseError::InvalidLineEndings)) - 1;
		if Some(expected_cr_position) == message_text.find('\r') {
			Ok(CRLF)
		} else {
            // either we found an \r somewhere else in the body or we didn't find it at all
			Ok(LF)
		}
	}
	
	fn parse_headers(mut headers_text : &'a mut [u8], line_ending_format: LineEndingFormat) -> Result<HashMap<&'a str, &'a str>, ParseError> {
//		let mut header_fields = headers_text.as_bytes().splitn_mut(2, |b| *b == b'a');
//
		let mut headers : HashMap<&'a str, &'a str> = HashMap::new();
//		
//		let mut s = "asdf".to_string();
//		
//		let mut h = ["a".to_string(), "b".to_string()];
//		
//        let vec = vec![1, 2, 3];
//        let int_slice = &vec[1..2];
		
//		let header_field : &mut str = str::split(&mut s, "a").next().unwrap();
		
//		for mut header_field in &mut s.split("a") {
//			let name_and_value = header_fields.nth(0).unwrap().splitn(2, ":");
//			let name : &mut str = name_and_value.next().ok_or(ParseError::InvalidHeaders).unwrap();
//			header_fields.make_ascii_lowercase();
//			let value = try!(name_and_value.next().ok_or(ParseError::InvalidHeaders));
//			headers.insert(name.trim(), value.trim());
//		}

		Ok(headers)
	}

	pub fn from_buffer(buf: &'a mut [u8]) -> Result<Message<'a>, ParseError> {
//		// Convert the buffer to string
//		let message_text : &mut str = try!(str::from_utf8(&mut buf).map_err(ParseError::InvalidEncoding));
//
//		let line_ending_format = try!(Message::get_line_endings(message_text));
//
//		// Retrieving the Initial Line
//		let mut split_by_initial_line = message_text.splitn(2, line_ending_format.line_separator);
//		let initial_line : &mut str = try!(split_by_initial_line.next().ok_or(ParseError::InvalidLineEndings));
//		
//		// Parsing Headers
//		let headers_and_body = try!(split_by_initial_line.next().ok_or(ParseError::InvalidLineEndings));
//		let mut split_headers_and_body = headers_and_body.splitn(2, line_ending_format.line_separator);
//		let header_text : &mut str = try!(split_headers_and_body.next().ok_or(ParseError::InvalidLineEndings));
//		let headers = try!(Message::parse_headers(&mut header_text, line_ending_format));
//
//		Ok(Message {initial_line: initial_line, headers: headers})
		Err(ParseError::InvalidHeaders)
	}
}

#[test]
fn from_buffer_initial_line() {
	let mut buf_mut = [0u8; 1000];
	let buf = b"random info\r\nHost: localhost";
	let message = Message::from_buffer(&mut buf_mut).unwrap();
	
	assert_eq!(message.initial_line.to_string(), "random info".to_string());
}

#[test]
fn from_buffer_headers() {
	let mut buf_mut = [0u8; 1000];
	let buf = b"random info\r\nHost: localhost\r\n\r\nbody";
	let message = Message::from_buffer(&mut buf_mut).unwrap();
	
	assert_eq!(message.headers.get("host").unwrap().to_string(), "localhost".to_string());
}

#[test]
fn get_line_endings() {
	let invalid_sample = "Some Random Info";

	match Message::get_line_endings(invalid_sample) {
		Err(ParseError::InvalidLineEndings) => assert!(true),
		_ => assert!(false)
	}

	let crlf_line_endings_sample = "Some Random Info\r\nMoreInfo\r\n\r\r\n";
	assert_eq!(Message::get_line_endings(crlf_line_endings_sample).unwrap().line_separator, CRLF.line_separator);

	let lf_line_endings_sample = "Some Random Info\nMoreInfo\n\n";
	assert_eq!(Message::get_line_endings(lf_line_endings_sample).unwrap().line_separator, LF.line_separator);
}