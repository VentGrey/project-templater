use scanrs::scanln;

pub fn init() {
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
        "1" => {},
        "2" => {},
        "3" => {},
        _ => {}
    }


}
