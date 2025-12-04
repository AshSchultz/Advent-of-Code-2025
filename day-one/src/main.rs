use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
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
    let mut total_count: i32 = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(input_filename) {
        let mut curr_pointing = 50;
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            turn_knob(&mut curr_pointing, line, &mut total_count);
        }
    }
    println!("{total_count}");
}

fn turn_knob(curr_pointer:& mut i32, mut input:String, tot_count: &mut i32) {
    let rotation_value_s = input.split_off(1);   
    let rotation_value:i32 = rotation_value_s.parse().unwrap();

    if input.contains("L") {
        *curr_pointer -= rotation_value;
    } else {
        *curr_pointer += rotation_value;
    }

    if *curr_pointer < 0 {
        *curr_pointer = *curr_pointer + 100*(rotation_value/100);
    } else if *curr_pointer > 100 {
        *curr_pointer = *curr_pointer - 100*(rotation_value/100);
    }

    if *curr_pointer == 0 {
        *tot_count += 1;
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}