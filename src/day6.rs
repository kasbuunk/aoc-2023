pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    Ok((sum_a as u32, sum_b as u32))
}

fn puzzle_a(data: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let races = extract_races(data);

    let combinations_to_win: u64 = races
        .iter()
        .map(|x| compute_ways_to_beat_record(x))
        .product();

    Ok(combinations_to_win as u64)
}

fn puzzle_b(data: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let race = extract_race(data);

    Ok(compute_ways_to_beat_record(&race))
}

struct Race {
    time: u64,
    distance: u64,
}

fn extract_races(data: &str) -> Vec<Race> {
    let races_data: Vec<Vec<u64>> = data
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|number| number.parse::<u64>().expect("data is not a number"))
                .collect()
        })
        .collect();

    races_data[0]
        .iter()
        .zip(races_data[1].iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect()
}

fn extract_race(data: &str) -> Race {
    let race_data: Vec<u64> = data
        .trim()
        .lines()
        .map(|line| {
            line.split_terminator(':')
                .skip(1)
                .map(|line| line.replace(" ", ""))
                .map(|number| number.parse::<u64>().expect("data is not a number"))
                .next()
                .expect("no number found")
        })
        .collect();

    Race {
        time: race_data[0],
        distance: race_data[1],
    }
}

fn compute_ways_to_beat_record(race: &Race) -> u64 {
    (0..race.time)
        .filter(|milliseconds_waited| {
            (race.time - milliseconds_waited) * milliseconds_waited > race.distance
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected_output_a: u64,
        expected_output_b: u64,
    }

    #[test]
    fn puzzle() {
        let test_cases = vec![TestCase {
            input: "Time:      7  15   30
Distance:  9  40  200"
                .into(),
            expected_output_a: 288,
            expected_output_b: 71503,
        }];

        for test_case in test_cases {
            let output = puzzle_a(&test_case.input).expect("solving puzzle a");
            assert_eq!(
                output, test_case.expected_output_a,
                "input: {}",
                test_case.input,
            );

            let output = puzzle_b(&test_case.input).expect("solving puzzle b");
            assert_eq!(
                output, test_case.expected_output_b,
                "input: {}",
                test_case.input,
            );
        }
    }
}
