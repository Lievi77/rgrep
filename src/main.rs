use std::env;
use std::fs;

fn main() {
    //First, we need to collect the arguments given to the program
    let args: Vec<String> = env::args().collect(); //collect returns an iterator (vector)

    let config = parse_config(&args);

    let error_msg = format!("Could not read file '{}'", config.filename);
    let contents = fs::read_to_string(config.filename).expect(&error_msg); //read the contents of the input filename.

    //.expect prints the error message in the input instead of the standard panic! message
    println!("{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
