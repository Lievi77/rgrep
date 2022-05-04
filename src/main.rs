use std::env;

fn main() {
    //First, we need to collect the arguments given to the program
    let args: Vec<String> = env::args().collect(); //collect returns an iterator (vector)

    let query = &args[1];
    let file_name = &args[2];
}
