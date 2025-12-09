use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_filename = get_input_filename();
    // let part_one: i32 = calc_part_one(input_filename.clone());
    let part_two: i32 = calc_part_two(input_filename.clone());
    // println!("Part 1: {part_one}");
    println!("Part 2: {part_two}");

}

fn calc_part_two(input_filename: String) -> i32 {
    let mut total_count: i32 = 0;
    let mut curr_pointing: i32 = 50;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(input_filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            turn_knob(&mut curr_pointing, line, &mut total_count);
        }
    }
    return total_count;
}

// fn calc_part_one(input_filename: String) -> i32 {
//     let mut total_count: i32 = 0;
//     let mut curr_pointing: i32 = 50;
//     // File hosts.txt must exist in the current path
//     if let Ok(lines) = read_lines(input_filename) {
//         // Consumes the iterator, returns an (Optional) String
//         for line in lines.map_while(Result::ok) {
//             println!("{}", line);
//             turn_knob(&mut curr_pointing, line, &mut total_count);
//         }
//     }
//     return total_count;
// }

fn turn_knob(curr_pointer: &mut i32, mut input: String, tot_count: &mut i32) {
    let rotation_value_s = input.split_off(1);
    let rotation_value: i32 = rotation_value_s.parse().unwrap();
    let dir = input.contains("L");
    calc_crossed_zero(curr_pointer, rotation_value, tot_count, dir);
}


fn calc_crossed_zero(curr_pointer: &mut i32, rot_value: i32, tot_count: &mut i32, dir: bool) {
    let mut i = 0;
    while i < rot_value {
        if dir {
            *curr_pointer -= 1;
        } else {
            *curr_pointer += 1;
        }
        if *curr_pointer < 0 {
            *curr_pointer = 99;
        } else if *curr_pointer > 99 {
            *curr_pointer = 0;
        }
        if *curr_pointer == 0 {
            *tot_count += 1;
        }
        i += 1;
    }
    println!("Direction: {}      Rotation value: {}      current pointer: {}     total count: {}", if dir {"L"} else {"R"}, rot_value, *curr_pointer, *tot_count);
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
