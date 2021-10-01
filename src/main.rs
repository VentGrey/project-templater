use std::env;
use std::process::exit;

/* Modules */
mod cproj;
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
    let ver: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    let aut: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");

    println!(
        "project-templater ({})
Deliver your college projects with style!
Authors: [{}]
License: [GPL-2.0]
USAGE:
    ptm [ARGS]
ARGS:
    new, --new, -new or -n        Creates a new project
    help, --help, -help or -h     Displays this message
For more information about the development of this program please see
the README file included in:
https://github.com/VentGrey/project-templater/blob/master/README.md",
        ver.unwrap_or("unknown"),
        aut.unwrap_or("none"),
    );
}
