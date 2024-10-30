mod day01;
mod day02;
mod day03;
mod day06;

// TODO: add a decition tree, CLI interface, or something such that everything isn't just run in sequentional order.
fn main() {
    let mut daychoice: i32 = 0;

    daychoice = 0;
    match daychoice {

        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        
        6 => day06::day06(),

        _ => println!("No valid choice given"),
    }
}
