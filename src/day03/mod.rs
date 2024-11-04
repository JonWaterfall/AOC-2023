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
 */

use std::{path::Path, vec};

use regex::Regex;

#[path = "../utility.rs"]
mod utility;



pub fn day03() {
    let input_file = Path::new("src/inputs/day03exampleinput.txt");
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

    // will match with ".", single number, and any non-word symbol(but not ".").
    let sym_regex = Regex::new(r"(?<dot>[\.])|(?<num>[\d])|(?<symbol>[^\.\w\s])").unwrap();

    

    println!("Line len is: {}", line_len);

    // how to find "up" from a given position ?
    // idea: find length of previous line. Then add length between £ and prevous lineshift to index of lineshift before that again. (assuming we don't step out of bounds)
    let mut line_num: i32 = 0;
    for line in input_string.lines(){
        for sym in sym_regex.captures_iter(line).enumerate() {
            print!("{}:", sym.0);
            //print!("{}:", sym.1.get(0).unwrap().start()); //byte offset of match. A byte offset is NOT synonymus with character offset in UTF-8
            print!("{}   ", sym.1.get(0).unwrap().as_str());
            //let x = sym.get(0);
        }
        println!("");
        line_num += line_num;
    }
    
    return 5; //temp while I write function
    
}




 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input_file_and_solve_01() {
        let example_input_file = Path::new("src/inputs/day03exampleinput.txt");

        assert_eq!(solve_01(utility::read_input_file(example_input_file).unwrap()), 4361);
    }
}