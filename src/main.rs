use std::{fs::File, io::Read};

mod day1;
mod day2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (day1a, day1b) = crate::day1::solution(&load_data("day1")?)?;
    let (day2a, day2b) = crate::day2::solution(&load_data("day2")?)?;
    let solutions = format!(
        "Solutions:
1A: {}
1B: {}
2A: {}
2B: {}
",
        day1a, day1b, day2a, day2b,
    );
    println!("{}", &solutions);

    Ok(())
}

fn load_data(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data_directory = "data/";
    let mut file = File::open(format!("{}{}", data_directory, file_name))?;
    let mut data = "".into();
    let _ = file.read_to_string(&mut data)?;

    Ok(data)
}
