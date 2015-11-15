use std::collections::HashMap;

pub struct Request<'a> {
	/// The HTTP version of the Client. Most likely '1.0' or '1.1'.
	http_version: &'a str,
	/// Lowercased headers without duplicates
	headers: HashMap<&'a str, &'a str>,
	trailers: HashMap<&'a str, &'a str>,
	/// The name of the request method in all caps. GET, POST... etc
	method: &'a str,
	url: &'a str, 
}

impl<'a> Request<'a> {
	/// Initializes a request with empty values
	fn new() -> Request<'a> {
		Request {
			http_version: "",
			headers: HashMap::new(),
			trailers: HashMap::new(),
			method: "",
			url: ""
		}
	}
	
	fn from_buffer(buf: &mut [u8]) -> Request<'a> {
		let request = Request::new();
		
		request
	}
}