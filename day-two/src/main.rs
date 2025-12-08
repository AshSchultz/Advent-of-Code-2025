
use std::io;

fn main() {
    let filename = "./input-file/input.txt".to_string(); //get_filename();
	let sum_part_one = part_one(&filename);
	let sum_part_two = part_two(&filename);
    println!("part one: {sum_part_one}");
	println!("part two: {sum_part_two}");
}

fn get_filename() -> String {
	let mut input_filename: String = String::new();
	println!("Enter path to the challenges input file relative to the current path");
    match io::stdin().read_line(&mut input_filename) {
        Ok(n) => {
            println!("{n} bytes read");
            input_filename = input_filename.strip_suffix("\n").unwrap().to_string();
            println!("{input_filename}");
        }
        Err(error) => println!("error: {error}"),
    }

	return input_filename;
}

fn part_one(input_file: &String) -> u128 {
	let contents = match std::fs::read_to_string(input_file) {
		Ok(file) => file,
		Err(error) => panic!("Problem reading file: {:?}", error)
	};
    let parts: Vec<&str> = contents.split(',').collect();
	let mut sum = 0;
	for range in parts {
		println!("{}", range);
		for item in evalute_range(range) {
			sum += item as u128;
		}
	}
	return sum;
}

fn evalute_range(range: &str) -> Vec<u128> {
	let mut full_range: Vec<&str> = range.split('-').collect();
	// 422394377-422468141
	for i in 0 .. full_range.len(){
		full_range[i] = full_range[i].trim();
	}
	let range_start: u128 = full_range[0].parse().expect("Failed to parse start of range");
	let range_end: u128 = full_range[1].parse().expect("Failed to parse end of range");
	// 4949410945-4949555758
	println!("Evaluating range from {range_start} to {range_end}");
	let mut collection: Vec<u128> = [].to_vec();
	for i in range_start .. range_end {
		let mut last_half = i.to_string();
		let first_half = last_half.split_off((last_half.len())/2);
		if first_half == last_half {
			collection.push(i);
		}
	}
	return collection;
}

fn part_two(input_file: &String) -> u128 {
	let contents = match std::fs::read_to_string(input_file) {
		Ok(file) => file,
		Err(error) => panic!("Problem reading file: {:?}", error)
	};
    let parts: Vec<&str> = contents.split(',').collect();
	let mut sum = 0;
	for range in parts {
		println!("{}", range);
		for item in evalute_range_part_two(range) {
			sum += item as u128;
		}
	}
	return sum;
}

fn evalute_range_part_two(range: &str) -> Vec<u128> {
	let mut full_range: Vec<&str> = range.split('-').collect();
	// 422394377-422468141
	for i in 0 .. (full_range.len()) {
		full_range[i] = full_range[i].trim();
	}
	let range_start: u128 = full_range[0].parse().expect("Failed to parse start of range");
	let range_end: u128 = full_range[1].parse().expect("Failed to parse end of range");
	// 4949410945-4949555758
	println!("Evaluating range from {range_start} to {range_end}");
	let mut collection: Vec<u128> = [].to_vec();
	for i in range_start .. range_end {
		let id_curr: String = i.to_string();
		// take substrings until half way through the string
		for j in 1 .. ((id_curr.len())/2) {
			// Get the substring
			let pat = id_curr.get(0..j).unwrap().to_string();
			// get the rest of the string
			let rest_of_str = id_curr.get(j..id_curr.len()).unwrap().to_string();
			// Pass the substring and rest of the string into compare to end
			if compare_to_end(pat, rest_of_str) {
				collection.push(i);
				break;
			}
		}
	}
	return collection;
}

// Compare the pattern to the rest of the string, patterns size by patterns size
fn compare_to_end(pattern: String, rest_of_string: String) -> bool {
	let size = pattern.len();
	let iters = rest_of_string.len();
	let mut i = 0;
	while i < iters {
		let next_string_bit= rest_of_string.get(i .. i+size).unwrap();
		if pattern != next_string_bit {
			return false;
		}
		i += size;
	}

	return true;
}