use std::env;
use std::process::exit;

/* Modules */
mod project;


/*
 *
 * Main function that handles arguments and dependencies on how to
 * proceed.
 *
 * NOTE: All additional projects should be done in a module.*/
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("{}", "Excess arguments!");
        exit(1);
    } else if args.len() == 1 {
        println!("{}", "Empty arguments list!");
        exit(1);
    } else {
        let option: &String = &args[1];

        match option.as_str() {
            "new" | "--new" | "-new" | "-n" => project::init(),
            "help" | "--help" | "-help" | "-h" => help(),
            _ => {
                println!("Invalid argument...");
                exit(1);
            }
        }
    }
}

fn help() {

}
