/*
 * input has too much style added by junior fronend dev.
 * each line in the input uses the first number from the left and first number of the right to get a two-digit number, even if there's only one or more than two numbers in each line
 * all two-digit numbers are added together, and that is the answer for part 1
 * 
 * Part 2 adds complexity by having to considder number wrtten as word as well
 */

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn day01() {
    let input_file = Path::new("src/inputs/day01input.txt");

    let useful_input_data = read_input_file_01(input_file);
    println!("The initial sum of all calibration values are: {}", sum_of_vector(useful_input_data));

    println!("The second sum of all calibration values are: {}", sum_of_vector(read_input_file_02(input_file)));
}

fn sum_of_vector(list_of_numbers: Vec<u8>) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..list_of_numbers.len() {
        sum += u32::from(list_of_numbers[i]);
    }
    return sum;
}

// reads input file as described in part 1 of the day01 puzzle
fn read_input_file_01(input_filepath: &Path) -> Vec<u8> {
    let mut ret_vec: Vec<u8> = Vec::new();
    let mut file: File = match File::open(&input_filepath) {
        Err(why) => panic!("Could not open day06 input file: {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read file to string : {}", why),
        Ok(_) => ()
    }

    for line in s.lines() {
        let first_index = line.find(|c: char| c.is_digit(10));
        if first_index.is_none() {
            panic!("Input file had a line without a number!");
        }
        // NOTE: I think this is ugly. If input file somehow has non-ASCII I expect this to crash the program.
        // NOTE2: returned 49 and 50. Those are the decimal values for '1' and '2' in ASCII. I'm not sure how to convert a value of and ASCII to the actual character or number, so lets do something else
        // let first_number: u8 = line.as_bytes()[first_index.unwrap()];
        // let last_number: u8 = line.as_bytes()[line.rfind(|c: char| c.is_digit(10)).unwrap()];

        // new version. need to convert char to int again. It's an iterator accoring to stack overflow but we don't care about being o(n).
        let first_number = line.chars().nth(first_index.unwrap()).unwrap();
        let last_number = line.chars().nth(line.rfind(|c: char| c.is_digit(10)).unwrap()).unwrap();
        //println!("First number is {} \nSecond number is {}", first_number, last_number);

        let mut concat_str = String::from(first_number);
        concat_str.push(last_number);
        let concat_num = match concat_str.parse::<u8>() {
            Err(why) => panic!("Could not parse the concatinated numberstring to a number: {}", why),
            Ok(n) => n
        };

        ret_vec.push(concat_num);
    }

    return ret_vec;
}

// reads input file as amended in part 2. Mostly just a copy-paste of the first function with added pattern matching
fn read_input_file_02(input_filepath: &Path) -> Vec<u8> {
    let mut ret_vec: Vec<u8> = Vec::new();

    let mut file: File = match File::open(&input_filepath) {
        Err(why) => panic!("Could not open day06 input file: {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read file to string : {}", why),
        Ok(_) => ()
    }

    // replace written numbers with digits. 
    // Although the example had the word "sixteen" in it, I have been unable to find any words above "nine" in my provided input.
    // Edgecase: "twone" appears once in the example and 18 times in my input. The text says nothing on if combined words are just one of two number or both.
    // "eightwo", "oneight" also appears 19 and 9 times in my input. This is a very artsy elf that "prettified" the input.
    s = s.replace("eightwo", "82");
    s = s.replace("twone", "21");
    s = s.replace("oneight", "18"); 
    // Realizing that all three can be combined. But can't find any in my input. 
    // This is a weakness in my chosen approach. Better alternative would have been to expand the find() pattern

    s = s.replace("sixteen", "16");

    s = s.replace("one", "1");
    s = s.replace("two", "2");
    s = s.replace("three", "3");
    s = s.replace("four", "4");
    s = s.replace("five", "5");
    s = s.replace("six", "6");
    s = s.replace("seven", "7");
    s = s.replace("eight", "8");
    s = s.replace("nine", "9");
    s = s.replace("ten", "10");
    

    for line in s.lines() {
        let first_index = line.find(|c: char| c.is_digit(10));
        if first_index.is_none() {
            panic!("Input file had a line without a number!");
        }
        let first_number = line.chars().nth(first_index.unwrap()).unwrap();
        let last_number = line.chars().nth(line.rfind(|c: char| c.is_digit(10)).unwrap()).unwrap();

        let mut concat_str = String::from(first_number);
        concat_str.push(last_number);
        let concat_num = match concat_str.parse::<u8>() {
            Err(why) => panic!("Could not parse the concatinated numberstring to a number: {}", why),
            Ok(n) => n
        };

        ret_vec.push(concat_num);
    }

    return ret_vec;
}











#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_vectors(){
        let test_input: Vec<u8> = [12,38,15,77].to_vec();
        assert_eq!(sum_of_vector(test_input), 142);
        // Part 2:
        assert_eq!(sum_of_vector([29,83,13,24,42,14,76].to_vec()), 281);
    }

    #[test]
    fn test_read_input_file_01() {
        let example_input_file = Path::new("src/inputs/day01exampleinput.txt");

        assert_eq!(read_input_file_01(example_input_file), [12,38,15,77].to_vec());
    }

    #[test]
    fn test_read_input_file_02() {
        let example_input_file = Path::new("src/inputs/day01_2_exampleinput.txt");
        let output_vec = read_input_file_02(example_input_file);

        assert_eq!(output_vec, [29,83,13,24,42,14,76].to_vec());
    }
}