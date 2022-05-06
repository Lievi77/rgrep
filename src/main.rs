use std::env;
use std::fs;
use std::process;

fn main() {
    //First, we need to collect the arguments given to the program
    let args: Vec<String> = env::args().collect(); //collect returns an iterator (vector)

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); //exit 0 for normal execution termination
                          //exit {number other than 0} for abnormal execution termination.
    });

    let error_msg = format!("Could not read file '{}'", config.filename);
    let contents = fs::read_to_string(config.filename).expect(&error_msg); //read the contents of the input filename.

    //.expect prints the error message in the input instead of the standard panic! message
    println!("{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        //Err values will always have a 'static lifetime
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
