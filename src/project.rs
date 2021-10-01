use std::process::{Command, Stdio, exit};

use scanrs::scanln;

use crate::cproj;

pub fn init() {

    if !compatible() {
        println!(
            "Your system is NOT compatible with this program!",
        );
        exit(1);
    }

    println!("Starting new project, please select your desired options:");
    println!(
        "What kind of project would you like to setup?

        1) C project (c17 - standard)
        2) Python 3 project
        3) Rust project

Please enter your desired project in the line below (1, 2 or 3):
");

    let opt: String = scanln();

    match opt.as_str() {
        "1" => {
            check_deps(1);
            cproj::init();
        },
        "2" => {},
        "3" => {},
        _ => {}
    }
}


fn check_deps(lang: u8) {

}

fn compatible() -> bool {
    let compat: bool = if cfg!(target_os = "windows") {
        eprintln!("{}", "Sorry Windows isn't compatible");
        false
    } else if cfg!(target_os = "linux") {
        println!("{}", "Checking for wget...");
        // Check if wget is installed on this system
        let is_wget = Command::new("which")
            .arg("wget")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Could not run which");

        println!("{}", "Checking for tar...");
        // Check if tar is installed on this system
        let is_tar = Command::new("which")
            .arg("tar")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Could not run which");

        if (is_wget.code() != Some(0)) && (is_tar.code() != Some(0)) {
            println!(
                "Charlatan needs {} and {} to be installed",
                "wget",
                "tar"
            );
            return false;
        }

        println!("[{}] Your system is compatible", "✔");
        true
    } else {
        eprintln!("Could not determine compatibility with your OS. Sorry");
        false
    };

    compat
}
