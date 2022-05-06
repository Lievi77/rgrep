use std::env;
use std::fs;

fn main() {
    //First, we need to collect the arguments given to the program
    let args: Vec<String> = env::args().collect(); //collect returns an iterator (vector)

    let (query, filename) = parse_config(&args);

    let error_msg = format!("Could not read file '{}'", filename);
    let contents = fs::read_to_string(filename).expect(&error_msg); //read the contents of the input filename.

    //.expect prints the error message in the input instead of the standard panic! message
    println!("{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
