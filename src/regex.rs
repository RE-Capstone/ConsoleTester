use regex::RegexSet;
use std::str;

pub fn check_stream_for_escape(escapes: Vec<Vec<u8>>, user_string: &str) -> Vec<usize>
{
	let mut sequences = Vec::new();
	for v in escapes.iter() {
		sequences.push(str::from_utf8(&v).unwrap());
	}
	
	//let set = RegexSet::new(&sequences).unwrap();
	//let matches: Vec<usize> = set.matches(user_string).into_iter().collect();
    //matches
	RegexSet::new(&sequences).unwrap().matches(user_string).into_iter().collect()
}


// 'cargo test'
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_escape_test() {
		let test_data = vec![vec![103,114,101,97,116], vec![116,101,115,116], vec![107,107], vec![108,101,110,100]];
		let test_str = "o great string of testing, lend us your matches";
		assert_eq!(check_stream_for_escape(test_data, test_str), vec![0, 1, 3]);
	}
}




