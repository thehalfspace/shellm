// INTRO HERE

// Read the cli args using this crate
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let prompt = &args[1];
        println!("The input string is: {}", prompt);
    } else {
        eprintln!(
            "Usage: <input text string within quotes>"
        );
    };


}
