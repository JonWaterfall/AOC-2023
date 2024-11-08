/*
 *
 * idea 1: Seaching for non-"." symbols first then check for digits in the 8 adjasent tiles
 * track the position of each digit in a list then bind with connected digits
 * discard numbers with matching start and end positions
 * (similar to how a video game would check for relevant data)
 *  
 * idea 2: Use regex to find position of all digits and symbols
 * then use a position algorithm to determine relevant digits
 * (probably simpler to implement)
 * 
 * Both ideas are similar with the difference being if we wait for gathering the numbers or gather them at the same time as the symbols.
 * The difference between the two ideas depending entirely on how big the input is and how many "lonely" symbols and digits there are.
 * 
 * idea 3: What about linear search, or "doing it the hard way"?
 * More control over data gathering, but also needing to spend time finding out how to identify line breaks.
 * 
 * Reflection 1: what constitutes a "symbol" is ambigious. Is it anything that is non-alpha-numeric and not part of the english alphabet? 
 * For this exercise I will assume whatever is not defined as a "word character"  by Regex to be good enough for a first pass.
 * 
 * !!!! Reflection 2: !!!!
 * Apparantly, the Regex API for Rust does NOT currently implement a method for getting the character index of a match. 
 * It's not by character count, but by the match count or byte offset from haystack start.
 * Rust bros, I'm sorry but C# still got a leg over us here.
 * So what we then do is .enumerate() on captures_iter then check if 1.capture.get(0) is a number or a symbol. 
 * This is becoming a very ghetto linear search.
 * 
 * reflection 3: ended up getting a number that's "too high" but my test passes. So I need to scout for an edge case in my input file
 */

use std::path::Path;

use regex::Regex;

#[path = "../utility.rs"]
mod utility;



pub fn day03() {
    let input_file = Path::new("src/inputs/day03input.txt");
    let file_content = match utility::read_input_file(input_file) {
        Err(why) => panic!("Could not open day06 input file: {}", why),
        Ok(file) => file,
    };
    let answer_01 = solve_01(file_content);
    println!("Part 1 solution: {}", answer_01);
}

fn solve_01(input_string: String) -> u32 {
    solve_01_regex(input_string)
}


fn solve_01_regex(input_string: String) -> u32 {
    // assume all lines are the same size for simplicity
    let line_len: i32 = input_string.lines().next().unwrap().len().try_into().unwrap();
    let line_count: i32 = input_string.lines().count() as i32;

    // will match with ".", single number, and any non-word symbol(but not ".").
    let sym_regex = Regex::new(r"(?<dot>[\.])|(?<num>[\d])|(?<symbol>[^\.\w\s])").unwrap();


    // Pair denotes Location, value
    let mut num_col: Vec<(std::ops::Range<i32>, i32)> = Vec::new();
    let mut sym_locations: Vec<i32> = Vec::new();


    // Parse / data collection
    collect_symbols_and_numbers(input_string, sym_regex, line_len, &mut num_col, &mut sym_locations);


    // Itterate over our numbers and see if symbols are near them.
    let mut answer: u32 = 0; 
    'nums: for number in num_col {
        let left_bound = if number.0.start % line_len == 0{
            number.0.start
        } else {
            number.0.start -1
        };
        let right_bound = if number.0.end % line_len == line_len-1{
            number.0.end
        } else {
            number.0.end +1
        };
        //println!("Left bound: {}, Right bound: {}", left_bound, right_bound);

        // look on line above
        if left_bound >= line_len {
            for n in left_bound-line_len..=right_bound-line_len { // remember to use inclusive range
                if sym_locations.contains(&n){
                    answer += number.1.unsigned_abs(); // lazy casting. We are guaranteed to be positive anyway.
                    //println!("num {} found near pos {}", number.1.unsigned_abs(), n);
                    continue 'nums;
                };
            }
        }
        // look on line below
        if (right_bound + line_len >= line_len * line_count) == false { // line_len * line_count is out of bounds by one
            for n in left_bound+line_len..=right_bound+line_len {
                if sym_locations.contains(&n){
                    answer += number.1.unsigned_abs();
                    //println!("num {} found near pos {}", number.1.unsigned_abs(), n);
                    continue 'nums;
                }
            }
        }
        if sym_locations.contains(&left_bound) || sym_locations.contains(&right_bound) {
            answer += number.1.unsigned_abs();
            //println!("num {} found near pos {}", number.1.unsigned_abs(), left_bound);
        }
    }
    
    return answer; //temp while I write function
    
}

// Goes through each line of the input_string and scans with sym_regex. Numbers and symbols are collected and placed in num_col and sym_locations.
// assumes regex to be formated to match with every character and use the naming groups "num" and "symbol" for numbers and symbols.
fn collect_symbols_and_numbers(input_string: String, sym_regex: Regex, line_len: i32, num_col: &mut Vec<(std::ops::Range<i32>, i32)>, sym_locations: &mut Vec<i32>) {
    let mut line_num: i32 = 0;
    for line in input_string.lines(){
        let mut num_flag: bool = false;
        let mut num_buffer: String = String::new();

        for sym in sym_regex.captures_iter(line).enumerate() {
            // collect numbers into a buffer
            let option_num = sym.1.name("num");
            if option_num.is_some() {
                if num_flag == false {
                    num_flag = true;
                }
                num_buffer.push_str(option_num.unwrap().as_str());
            }
            // when to stop collecting numbers
            if (option_num.is_none() || (sym.0+1) as i32 == line_len ) && num_flag { 
                // need to do some if elses for when the last character in the line is a number
                let low_range: i32 = if option_num.is_none() {
                    (sym.0 - num_buffer.len()) as i32
                    }
                    else {
                    (sym.0 - num_buffer.len()) as i32 + 1
                }; 
                let high_range: i32 = if option_num.is_none() {
                    sym.0 as i32 -1
                    }
                    else {
                    sym.0 as i32
                }; 
                let num_range: std::ops::Range<i32> = low_range+(line_num*line_len)..high_range+(line_num*line_len);
                let parsed_num: i32 = num_buffer.parse().unwrap();

                num_col.push((num_range, parsed_num));
                num_buffer.clear();
                num_flag = false;
            }
        
            // We only need the location of the symbols
            let option_sym = sym.1.name("symbol");
            if option_sym.is_some() {
                sym_locations.push(sym.0 as i32 + (line_num*line_len));
            }

        }
        
        line_num = line_num + 1;
    }
}




 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input_file_and_solve_01() {
        let example_input_file = Path::new("src/inputs/day03exampleinput.txt");

        assert_eq!(solve_01(utility::read_input_file(example_input_file).unwrap()), 4361);
    }

    #[test]
    fn test_read_input_file_and_solve_02() {
        let example_input_file = Path::new("src/inputs/day03exampleinput.txt");

        assert_eq!(solve_02(utility::read_input_file(example_input_file).unwrap()), 467835);
    }
}