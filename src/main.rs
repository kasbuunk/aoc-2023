mod day1;
mod day2;

fn main() {
    let (day1a, day1b) = crate::day1::solution().expect("day 1 failed");
    let solutions = format!(
        "Solutions:
1A: {}
1B: {}
",
        day1a, day1b
    );
    println!("{}", &solutions);
}
