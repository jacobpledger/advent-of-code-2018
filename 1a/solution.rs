// Author: Jacob Pledger
// I didn't quite understand at the time that the goal was merely to
// provide the answer to the puzzle, since it was presented with so
// much ceremony, so this prints out the pattern nicely like in the
// description.
use std::env;

fn main() {
    let mut current_frequency = 0;
    // Skip arg0 since it's the name of the executable
    for frequency_change in env::args().skip(1) {
        let frequency_change = match frequency_change.to_string().parse::<i8>() {
            Ok(frequency_change) => frequency_change,
            Err(_) => std::process::exit(0), // newline?
        };
        let next_frequency = current_frequency + frequency_change;
        let symbol = match frequency_change > 0 {
            true => "+",
            false => "",
        };
        println!(
            "Current frequency {}, change of {}{}; resulting frequency {}",
            current_frequency, symbol, frequency_change, next_frequency
        );
        current_frequency = next_frequency;
    }
}
