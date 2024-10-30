/* 
 * After using the same Read_File function setup so many times, I figured it was high time to set up a utility module
 * And learn to spesify module paths
 */

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::Error;

pub fn read_input_file(input_filepath: &Path) -> Result<String, Error> {
    // read the input file
    let mut file: File = match File::open(&input_filepath) {
        Err(why) => return Err(why), //panic!("Could not open input file: {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    return match file.read_to_string(&mut s) {
        Err(why) => Err(why), //panic!("Could not read file to string : {}", why),
        Ok(_) => Ok(s)
    }
}