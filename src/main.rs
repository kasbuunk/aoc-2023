use std::{fs::File, io::Read};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solutions = vec![
        crate::day1::solution(&load_data("day1")?)?,
        crate::day2::solution(&load_data("day2")?)?,
        crate::day3::solution(&load_data("day3")?)?,
        crate::day4::solution(&load_data("day4")?)?,
        crate::day5::solution(&load_data("day5")?)?,
        crate::day6::solution(&load_data("day6")?)?,
        crate::day7::solution(&load_data("day7")?)?,
        crate::day8::solution(&load_data("day8")?)?,
        crate::day9::solution(&load_data("day9")?)?,
    ];

    let solutions_report: String = solutions
        .iter()
        .enumerate()
        .map(|(index, (solution_a, solution_b))| {
            let day_number = index + 1;
            format!(
                "{}A: {}\n{}B: {}\n",
                day_number, solution_a, day_number, solution_b
            )
        })
        .collect();
    println!(
        "Solutions:
{}",
        solutions_report
    );

    Ok(())
}

fn load_data(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data_directory = "data/";
    let mut file = File::open(format!("{}{}", data_directory, file_name))?;
    let mut data = "".into();
    let _ = file.read_to_string(&mut data)?;

    Ok(data)
}
