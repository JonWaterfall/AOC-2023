# My Advent of Code 2023 solutions written in Rust
Exercising while learning a new language. 


## Code structure and how to run
All code is stored in the `src` folder, where `main.rs` also resides. Each Day(puzzle) solution are in their own folders labled `\dayXX` and contains a `mod.rs` with the code for arriving at the solutions. 

* Note: currently `main.rs` needs to be manually edited to execute a chosen day. 
* After editing the main file, you can run the code with `cargo run`.


## List of stars aqured so far

Day nr | stars
-|-
Day01 | **
Day02 | **
Day03 | *
.. | 
Day06 | **
.. | ..


## List of notable things I've spent some time thinking about
### Rust milti-file modules can be sturctured in several ways
I use the following two ways:

The first way it to create a folder with a file named `mod.rs`. The name of the folder will be the name of the module. 
This is used for each Day/puzzle and is ran from the `main.rs`file.

The second way is to make a .rs file with a public function and can be linked to with a tag like :`#[path = "../utility.rs"]`, which I do in Day03.

There are more ways to mess with the file hireachy and create headaches for maintainers. :)