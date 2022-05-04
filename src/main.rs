use std::env;
use std::fs;

fn main() {
    //First, we need to collect the arguments given to the program
    let args: Vec<String> = env::args().collect(); //collect returns an iterator (vector)

    let query = &args[1];
    let file_name = &args[2];

    let error_msg = format!("Could not read file '{}'", file_name);
    let contents = fs::read_to_string(file_name).expect(&error_msg); //read the contents of the input filename
    println!("{}", contents);
}
