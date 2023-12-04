pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    Ok((sum_a, sum_b))
}

fn puzzle_a(_data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(0)
}

fn puzzle_b(_data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        expected_output_a: u32,
        expected_output_b: u32,
    }

    #[test]
    fn puzzle() {
        let test_cases = vec![TestCase {
            input: "".into(),
            expected_output_a: 0,
            expected_output_b: 0,
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
