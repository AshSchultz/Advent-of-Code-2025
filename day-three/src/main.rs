use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
    let file_name = get_input_filename();
    let part_one = part_one(file_name);
    println!("{}", part_one);
}

fn part_one(filename: String) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let mut max_digit = 0;
            let mut max_digit_place = 0;
            let mut second_max_digit = 0;
            let mut digit_num = 0;
            for character in line.chars() {
                let digit = character.to_digit(10).unwrap();
                if digit > max_digit {
                    max_digit = digit;
                    max_digit_place = digit_num;
                }
                digit_num += 1;
            }
            digit_num = 0;
            for character in line.chars() {
                let digit = character.to_digit(10).unwrap();
                if digit_num > max_digit_place {
                    if second_max_digit < digit {
                        second_max_digit = digit;
                    }
                }
                digit_num += 1;
            }
        }
    }
    return 0;
}

fn get_input_filename() -> String {
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

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}