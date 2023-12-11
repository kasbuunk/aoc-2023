pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    Ok((sum_a as u32, sum_b as u32))
}

fn puzzle_a(data: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let history_rows = extract_history_rows(data);

    let sum = history_rows.iter().map(|row| predict_next_value(row)).sum();

    Ok(sum)
}

fn puzzle_b(data: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let history_rows = extract_history_rows(data);

    let sum = history_rows
        .iter()
        .map(|row| predict_previous_value(row))
        .sum();

    Ok(sum)
}

fn extract_history_rows(data: &str) -> Vec<Vec<i32>> {
    data.lines()
        .map(|row| {
            row.split_whitespace()
                .map(|number_str| {
                    number_str
                        .parse::<i32>()
                        .expect("found a string that was not a number")
                })
                .collect()
        })
        .collect()
}

fn predict_previous_value(row: &[i32]) -> i32 {
    let first_value = row.iter().nth(0).expect("row must have a first value");

    match row.iter().all(|number| *number == 0) {
        true => 0,
        false => {
            let next_row: Vec<i32> = vec![0]
                .iter()
                .chain(row.iter())
                .zip(row.iter())
                .map(|(first, next)| next - first)
                .collect();

            first_value - predict_previous_value(&next_row[1..])
        }
    }
}
fn predict_next_value(row: &[i32]) -> i32 {
    let last_value = row.iter().nth_back(0).expect("row must have a last value");

    match row.iter().all(|number| *number == 0) {
        true => 0,
        false => {
            let next_row: Vec<i32> = vec![0]
                .iter()
                .chain(row.iter())
                .zip(row.iter())
                .map(|(first, next)| next - first)
                .collect();

            last_value + predict_next_value(&next_row[1..])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected_output_a: i32,
        expected_output_b: i32,
    }

    #[test]
    fn puzzle() {
        let test_cases = vec![
            TestCase {
                input: "0 3 6 9 12 15".into(),
                expected_output_a: 18,
                expected_output_b: -3,
            },
            TestCase {
                input: "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
                    .into(),
                expected_output_a: 114,
                expected_output_b: 2,
            },
        ];

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
