use std::process::{exit, Command, ExitStatus, Stdio};

use scanrs::scanln;

use crate::cproj;

pub fn init() {
    if !compatible() {
        println!("Your system is NOT compatible with this program!");
        exit(1);
    }

    println!("Starting new project, please select your desired options:");
    println!(
        "What kind of project would you like to setup?

        1) C project (c17 - standard)
        2) Python 3 project
        3) Rust project

Please enter your desired project in the line below (1, 2 or 3):
"
    );

    let opt: String = scanln();

    match opt.as_str() {
        "1" => {
            let deps: Vec<String> = vec!["ccls".to_string(),
                                         "clang-format".to_string()].to_vec();
            check_deps(deps);
            cproj::init();
        }
        "2" => {}
        "3" => {}
        _ => {}
    }
}

fn check_deps(pkgs: Vec<String>) {
    for i in pkgs {
        println!("Checking for {}", i);
        let state: ExitStatus = Command::new("which")
            .arg(&i)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Could not run which");

        if state.code() != Some(0) {
            println!("{} is not installed on your system!", i);
            exit(127);
        }
    }
}

fn compatible() -> bool {
    let compat: bool = if cfg!(target_os = "windows") {
        eprintln!("Sorry Windows isn't compatible");
        false
    } else if cfg!(target_os = "linux") {
        true
    } else {
        eprintln!("Could not determine compatibility with your OS. Sorry");
        false
    };
    compat
}
