use std::collections::HashMap;
use std::str;

/// A generic http message that can be either a request or response
pub struct Message<'a> {
	/// This will be parsed by the respective message type (request or response)
	initial_line: &'a str,
	headers: HashMap<String, &'a str>,
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
	
	fn parse_headers(header_text : &'a str, line_ending_format: LineEndingFormat) -> Result<HashMap<String, &'a str>, ParseError> {
		let header_fields = header_text.split(line_ending_format.line_separator);

		let mut headers : HashMap<String, &'a str> = HashMap::new();
		
		for header_field in header_fields {
			let mut name_and_value = header_field.splitn(2, ":");
			let name = try!(name_and_value.next().ok_or(ParseError::InvalidHeaders));
			let value = try!(name_and_value.next().ok_or(ParseError::InvalidHeaders));
			headers.insert(name.trim().to_lowercase(), value.trim());
		}

		Ok(headers)
	}

	pub fn from_buffer(buf: &'a [u8]) -> Result<Message<'a>, ParseError> {
		// Convert the buffer to string
		let message_text = try!(str::from_utf8(buf).map_err(ParseError::InvalidEncoding));

		let line_ending_format = try!(Message::get_line_endings(message_text));

		// Retrieving the Initial Line
		let mut split_by_initial_line = message_text.splitn(2, line_ending_format.line_separator);
		let initial_line = try!(split_by_initial_line.next().ok_or(ParseError::InvalidLineEndings));
		
		// Parsing Headers
		let headers_and_body = try!(split_by_initial_line.next().ok_or(ParseError::InvalidLineEndings));
		let mut split_headers_and_body = headers_and_body.splitn(2, line_ending_format.line_separator);
		let header_text = try!(split_headers_and_body.next().ok_or(ParseError::InvalidLineEndings));
		let headers = try!(Message::parse_headers(header_text, line_ending_format));

		Ok(Message {initial_line: initial_line, headers: headers})
	}
}

#[test]
fn from_buffer_initial_line() {
	let buf = b"random info\r\nHost: localhost";
	let message = Message::from_buffer(buf).unwrap();
	
	assert_eq!(message.initial_line.to_string(), "random info");
}

#[test]
fn from_buffer_headers() {
	let buf = b"random info\r\nHost: localhost\r\n\r\nbody";
	let message = Message::from_buffer(buf).unwrap();
	
	assert_eq!(message.headers.get("host").unwrap().to_string(), "localhost");
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