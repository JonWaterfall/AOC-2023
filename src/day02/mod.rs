
// cube set: 12 red cubes, 13 green cubes, and 14 blue cubes
// determine legal/possible games
// sum the ID numbers 

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use regex::Regex;


struct CubeCollection {
    red_cubes  :i32,
    green_cubes:i32,
    blue_cubes :i32
}


pub fn day02() {
    let first_set_of_cubes = CubeCollection{red_cubes : 12, green_cubes: 13, blue_cubes:14};
    
    let input_file = Path::new("src/inputs/day02input.txt");

    println!("In the bag, there is a total of {} red cubes, {} green cubes, and {} blue cubes", first_set_of_cubes.red_cubes, first_set_of_cubes.green_cubes, first_set_of_cubes.blue_cubes);

    let valid_games = read_input_file_and_solve_01(input_file, first_set_of_cubes);
    println!("Valid game sum: {}", valid_games);
}


// very directly solve the puzzle, as I had trouble planning how to separate functionallity in a graceful manner.
fn read_input_file_and_solve_01(input_filepath: &Path, max_cubes: CubeCollection) -> i32 {
    // each game has several collections of cubes
    // example: "Game 1: 9 red, 2 green, 13 blue; 10 blue, 2 green, 13 red; 8 blue, 3 red, 6 green; 5 green, 2 red, 1 blue"
    let mut valid_game_sum: i32 = 0;
    let digit_regex = Regex::new(r"\d+").unwrap();
    let red_boxes_regex = Regex::new(r"(?<red>\d+) red").unwrap();
    let blue_boxes_regex = Regex::new(r"(?<blue>\d+) blue").unwrap();
    let green_boxes_regex = Regex::new(r"(?<green>\d+) green").unwrap();


    // read the input file
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
        // example line: "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"

        // rules: 
        // before colon will be "Game[blank]xxx" where xxx is the numeric ID of the game
        // after the colon are sub-games separated by semicolon
        // the order of cube colors seems to be random, and zero values are excluded in the input
        // box type separated by comma.

        // separating the line by colon and semicolon to make searches more targeted.
        let mut sep_line = line.split(&[':', ';'][..]);
        
        // assumption: the first number will always be the game ID, no need to check for "Game" as a string
        let first_part = sep_line.next().unwrap();
        // stored as a sting for now. It is a number.
        let game_id = digit_regex.find(first_part).unwrap().as_str();
        //println!("Game {}", game_id);
        let mut valid_game: bool = true;
        // 'for each' remaining part of the sep_line iterator we check if the cube number is greater than max_cubes 
        for sub_game in sep_line {
            // sub_game example: " 5 blue, 4 red, 13 green"
            //println!("boxes {}", sub_game);

            // use Regex to find r\d+\s*(color)
            let red_boxes = red_boxes_regex.captures(sub_game);
            if red_boxes.is_some() {
                //println!("Red boxes: {}", red_boxes.unwrap().name("red").unwrap().as_str());
                if max_cubes.red_cubes < FromStr::from_str(red_boxes.unwrap().name("red").unwrap().as_str()).unwrap() {
                    valid_game = false;
                    break; // I assume that the compiler cleans up the sep_line iterator.
                }
            }
            let green_boxes = green_boxes_regex.captures(sub_game);
            if green_boxes.is_some() {
                if max_cubes.green_cubes < FromStr::from_str(green_boxes.unwrap().name("green").unwrap().as_str()).unwrap() {
                    valid_game = false;
                    break;
                }
            }
            let blue_boxes = blue_boxes_regex.captures(sub_game);
            if blue_boxes.is_some() {
                if max_cubes.blue_cubes < FromStr::from_str(blue_boxes.unwrap().name("blue").unwrap().as_str()).unwrap() {
                    valid_game = false;
                    break;
                }
            }
        }
        if valid_game{
            valid_game_sum += i32::from_str(game_id).unwrap();
        }
        // else {
        //     valid_game = true;
        // }
    }

    return valid_game_sum;
}












#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input_file_and_solve_01() {
        let example_input_file = Path::new("src/inputs/day02exampleinput.txt");
        let first_set_of_cubes = CubeCollection{red_cubes : 12, green_cubes: 13, blue_cubes:14};

        assert_eq!(read_input_file_and_solve_01(example_input_file, first_set_of_cubes), 8);
    }

}