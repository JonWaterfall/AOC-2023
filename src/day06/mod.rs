/* 
 * Mini boat contest.
 * Time, speed, and distance is measured in whole units (milliseconds and millimeters)
 * Standing still at the start of the race increases speed additively by 1 millimeter per millisecond spent standing still.
 * The goal is to permutate through all possible outcomes and calculate how many of those oucomes would beat the current record holder for each race.
 * Then multiply all the ways you can beat each race together to get all possible ways to win the contest.
 * 
 * had to replace instances of 'i32' with 'u64' for second part of puzzle
 */

use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::cmp;
use std::path::Path;


pub fn day06() {
    let filepath = Path::new("src/inputs/day06input.txt");
    let race_data = read_input_file(filepath);
    let mut total_ways_to_beat_all_record = 1;

    // used to itterate over race_data directly before. switched to current style to make bvorrow checker happy.
    for n in 0..race_data.len() {
        let race_sols = beat_race_possibilities(race_data[n].0, race_data[n].1);
        print!("Race time: {} \t dist: {} \t has {} solutions.\n", race_data[n].0, race_data[n].1, race_sols);
        total_ways_to_beat_all_record *= race_sols;
    }
    println!("Mult of all solutions is: {}", total_ways_to_beat_all_record);


    // PART TWO
    // lmao bad kerning
    let race_data_for_real_this_time = anti_kern_touple_vector(race_data);
    let real_race_solution = beat_race_possibilities(race_data_for_real_this_time.0, race_data_for_real_this_time.1);
    println!("\nAfter anti-kerning the input, the solution is: {}", real_race_solution);
}

// calculates the number of ways a race can be beat 
fn beat_race_possibilities(time: u64, record_distance: u64) -> u64 {
    //let current_time = 0;
    //let current_speed = 0;
    let mut num_beatable_found = 0;

    // itteration of charging the boat. Not including the zero and last timestep because then you're just standing still at the starting line. 
    for charge_time in 1..time {
        //distance_reached = charge_time * time_left 

        // at first I used '>=' but the goal is to BEAT the current record and set a new one.
        if charge_time * (time - charge_time) > record_distance{
            num_beatable_found+=1;
        }
    }

    return num_beatable_found;
}

// return a vector of touples for time and record distance
fn read_input_file(input_filepath: &Path) -> Vec<(u64, u64)> {
    let mut race_records: Vec<(u64, u64)> = Vec::new();
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    // read file the naive way. For big files, a different approach should be used.
    let mut file = match File::open(&input_filepath) {
        Err(why) => panic!("Could not open day06 input file: {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read file to string : {}", why),
        Ok(_) => print!("String now contains: \n{}", s)
    }

    // parse string to respective vectors
    // original plan was to use Regex but ended up doing the simple approach instead.
    for line in s.lines() {
        if line.starts_with("Time:") {
            // collect all valid u64 parsable items after the first word and put it in our vector
            times = line[5..].split_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect();
        }
        else if line.starts_with("Distance:") {
            // same as above
            distances = line[9..].split_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect();
        }
        else if line.is_empty() {
            // do nothing. Not sure if EOF is counted as 'something'.
            break;
        }
        else {
            // unexpected format in file.
            // TODO: improve by skipping this step if times and distances already found their data. 
            eprintln!("Could not parse start of line. Found{}", line);
            exit(1);
        }
    }

    // pair the times and distances
    let pair_num = cmp::min(times.len(), distances.len());
    if pair_num <= 0 {
        return Vec::new();
    }
    else {
        for i in 0..pair_num {
            race_records.push((times[i],distances[i]));
        }
    }
    
    return race_records;
}


// in production, this would be considered a recipe for spagetti. But this is a puzzle
fn anti_kern_touple_vector(kerned_data: Vec<(u64,u64)>) -> (u64, u64) {
    let mut first = String::new();
    let mut second = String::new();

    for i in kerned_data {
        first.push_str(&i.0.to_string());
        second.push_str(&i.1.to_string()); //test driven development saved me from a misstyped number here
    }

    let first_num: u64 = match first.parse::<u64>() {
        Ok(number) => number,
        Err(e) => panic!("Could not parse string to int: {}", e)        
    };
    let second_num: u64 = match second.parse::<u64>() {
        Ok(number) => number,
        Err(e) => panic!("Could not parse string to int: {}", e)
    }; //throws error. Am I being too unsafe?
    return (first_num, second_num);
}



/// ****** TESTS ******
//TODO: Learn what is the most idiomatic way of organizing tests for multiple modules.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beat_race_possibilities_1() {
        assert_eq!(beat_race_possibilities(7, 9), 4);
    }
    #[test]
    fn test_beat_race_possibilities_2() {
        assert_eq!(beat_race_possibilities(15, 40), 8);
    }
    #[test]
    fn test_beat_race_possibilities_3() {
        assert_eq!(beat_race_possibilities(30, 200), 9);
    }
    #[test]
    fn test_muliply_race_possibilities() {
        assert_eq!(beat_race_possibilities(7, 9)*beat_race_possibilities(15, 40)*beat_race_possibilities(30, 200),288);
    }

    #[test]
    fn test_read_input_file_1() {
        let example_input_file = Path::new("src/inputs/day06exampleinput.txt");

        let output_data = read_input_file(example_input_file);
        assert_eq!(output_data[0], (7, 9));
        assert_eq!(output_data[1], (15, 40));
        assert_eq!(output_data[2], (30, 200));
    }

    #[test]
    fn test_anti_kern_touple_vector() {
        let example_input_file = Path::new("src/inputs/day06exampleinput.txt");
        let output_data = read_input_file(example_input_file);


        let anti_kern_data = anti_kern_touple_vector(output_data);
        assert_eq!(anti_kern_data.0, 71530);
        assert_eq!(anti_kern_data.1, 940200);
        assert_eq!(beat_race_possibilities(anti_kern_data.0, anti_kern_data.1), 71503)
    }
}