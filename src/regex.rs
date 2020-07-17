//use regex::RegexSet;
use regex::Regex;
use std::str;

//static WARNINGS: [u8; 32] = [0..31];

/*pub fn check_stream_for_escape(escapes: Vec<Vec<u8>>, user_string: &str) -> Vec<usize>
{
	let mut sequences = Vec::new();
	for v in escapes.iter() {
		sequences.push(str::from_utf8(&v).unwrap());
	}
	
	//let set = RegexSet::new(&sequences).unwrap();
	//let matches: Vec<usize> = set.matches(user_string).into_iter().collect();
	//matches
	RegexSet::new(&sequences).unwrap().matches(user_string).into_iter().collect()
}*/

pub fn remove_valid_escapes(escapes: Vec<Vec<u8>>, user_string: &str) -> String
{
	let mut result = String::from(user_string);
	for s in escapes.iter() {
		let re = Regex::new(str::from_utf8(&s).unwrap()).unwrap();
		result = re.replace_all(&result, "").to_string();
	}
	result
}

pub fn check_bad_escapes(user_string: &str) -> bool
{
	for c in user_string.chars() {
		if !c.is_ascii_control() {continue;}
		return false;
	}
	true
}


// 'cargo test'
#[cfg(test)]
mod tests {
	use super::*;

	/*#[test]
	fn check_escape_test() {
		//Vecs represent: "great", "test", "kk", "lend"
		let test_data = vec![vec![103,114,101,97,116], vec![116,101,115,116], vec![107,107], vec![108,101,110,100], vec![10]];
		let test_str = "o great string of testing,\n lend us your matches";
		assert_eq!(check_stream_for_escape(test_data, test_str), vec![0, 1, 3, 4]);
	}*/
	
	#[test]
	fn remove_valid_test() {
		let test_data = vec![vec![103,114,101,97,116], vec![116,101,115,116], vec![107,107], vec![108,101,110,100], vec![10]];
		let test_str = "o great string of testing,\n lend us your matches";
		
		assert_eq!("o  string of ing,  us your matches", remove_valid_escapes(test_data, test_str));
	}
	
	#[test]
	fn bad_escapes_test() {
		let test_data = vec![vec![10]];
		let test_str = "o great string of testing,\n lend us your matches";
		
		assert_eq!(false, check_bad_escapes(test_str));
		assert_eq!(true, check_bad_escapes(remove_valid_escapes(test_data, test_str).as_str()));
	}
	
	/*#[test]
	fn replacetest() {
		let test_data = vec![vec![103,114,101,97,116], vec![116,101,115,116], vec![107,107], vec![108,101,110,100], vec![10]];
		let mut result = String::from("o great string of testing,\n lend us your matches");
		for s in test_data.iter() {
			let re = Regex::new(str::from_utf8(&s).unwrap()).unwrap();
			result = re.replace_all(&result, "").to_string();
		}
		
		assert_eq!("o  string of ing,  us your matches", result);
		
	}*/
}




