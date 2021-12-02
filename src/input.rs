use std::env;
use std::fs::File;

pub fn read_input() -> Result<String, std::io::Error> {
    let input = env::args().nth(1).unwrap_or("-".into());

    // Try reading from file
    let mut file: Box<dyn std::io::Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
