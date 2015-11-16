#![feature(slice_patterns)]
use std::ops::IndexMut;
use std::vec::Vec;

#[test]
fn test_split_mut() {
    // A simple integer calculator:
    // `+` or `-` means add or subtract by 2
    // `*` or `/` means multiply or divide by 2
    let mut s = "asdf".to_string();
    let parts = split_at_first_mut(&mut s, "sd").unwrap();
    
    println!("part 1: {}", parts[0]);
    println!("part 2: {}", parts[1]);
    
    let a = [1, 2];
    
    let mut split_many_sample = "aa11aa11aa11aa";
}

fn split_at_first_mut<'b>(target: &'b mut str, separator: &'b str) -> Option<[&'b mut str; 2]> {
	if let Some(index) = target.find(separator) {
		let (first_part, second_part) = target.split_at_mut(index); 
		Some([first_part, second_part.index_mut(separator.len()..)])
	} else {
		None
	}
}

fn split_mut<'b>(target: &'b mut str, separator: &'b str) -> Option<Vec<&'b mut str>> {
    match split_mut(&mut target, separator) {
        Some(parts) => {
            parts.push(match split_at_first_mut(&mut target, separator) {
                Some([first_part, second_part]) => split_mut(&mut target, separator),
                None => target
            });
            parts
        },
        None => Vec::new() 
    };

    None
}
