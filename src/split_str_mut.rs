use std::str;
use std::ascii::AsciiExt;
use std::ops::IndexMut; 
use std::vec::Vec;

fn split_at_first_mut<'b>(target: &'b mut str, separator: &'b str) -> Option<[&'b mut str; 2]> {
	if let Some(index) = target.find(separator) {
		let (first_part, second_part) = target.split_at_mut(index); 
		Some([first_part, second_part.index_mut(separator.len()..)])
	} else {
		None
	}
}

fn split_mut<'b>(target: &'b mut str, separator: &'b str) -> Option<Vec<&'b mut str>> {
	None
}

#[test]
fn test_split_at_first_mut() {
	let mut sample = "Random12Info".to_string();
	
	let mut parts = split_at_first_mut(&mut sample, "12").unwrap();
	parts[0].make_ascii_lowercase();
	
	assert_eq!(parts[0].to_string(), "random".to_string());
	assert_eq!(parts[1].to_string(), "Info".to_string());
}
