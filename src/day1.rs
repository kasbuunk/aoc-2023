pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(data)?;
    let sum_b = puzzle_b(data)?;

    Ok((sum_a, sum_b))
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let sum: u32 = data
        .split_whitespace()
        .map(|x| {
            (x.chars()
                .filter(|c| c.is_numeric())
                .nth(0)
                .expect("first numeric character is missing")
                .to_digit(10)
                .expect("cannot convert first character to digit"))
                * 10
                + x.chars()
                    .filter(|c| c.is_numeric())
                    .nth_back(0)
                    .expect("last numeric character is missing")
                    .to_digit(10)
                    .expect("cannot convert last character to digit")
        })
        .sum();

    Ok(sum)
}

fn puzzle_b(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let sum: u32 = data
        .split_whitespace()
        .map(|x| string_to_numbers(x))
        .map(|x| x.iter().next().unwrap() * 10 + x.iter().next_back().unwrap())
        .sum();

    Ok(sum)
}

fn string_to_numbers(s: &str) -> Vec<u32> {
    let length = s.len();
    let mut list = vec![];
    let mut cursor = 0;

    loop {
        let number = match true {
            _ if s[cursor..].starts_with("0") => 0,
            _ if s[cursor..].starts_with("1") => 1,
            _ if s[cursor..].starts_with("2") => 2,
            _ if s[cursor..].starts_with("3") => 3,
            _ if s[cursor..].starts_with("4") => 4,
            _ if s[cursor..].starts_with("5") => 5,
            _ if s[cursor..].starts_with("6") => 6,
            _ if s[cursor..].starts_with("7") => 7,
            _ if s[cursor..].starts_with("8") => 8,
            _ if s[cursor..].starts_with("9") => 9,
            _ if s[cursor..].starts_with("one") => 1,
            _ if s[cursor..].starts_with("two") => 2,
            _ if s[cursor..].starts_with("three") => 3,
            _ if s[cursor..].starts_with("four") => 4,
            _ if s[cursor..].starts_with("five") => 5,
            _ if s[cursor..].starts_with("six") => 6,
            _ if s[cursor..].starts_with("seven") => 7,
            _ if s[cursor..].starts_with("eight") => 8,
            _ if s[cursor..].starts_with("nine") => 9,
            _ => {
                cursor = cursor + 1;
                if cursor >= length {
                    return list;
                }

                continue;
            }
        };

        cursor = cursor + 1;
        list.push(number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Day1Calibration {
        input: String,
        expected_output_a: u32,
        expected_output_b: u32,
    }

    #[test]
    fn test_day1() {
        let test_cases = vec![
            Day1Calibration {
                input: "".into(),
                expected_output_a: 0,
                expected_output_b: 0,
            },
            Day1Calibration {
                input: "00".into(),
                expected_output_a: 0,
                expected_output_b: 0,
            },
            Day1Calibration {
                input: "1".into(),
                expected_output_a: 11,
                expected_output_b: 11,
            },
            Day1Calibration {
                input: "12".into(),
                expected_output_a: 12,
                expected_output_b: 12,
            },
            Day1Calibration {
                input: "102".into(),
                expected_output_a: 12,
                expected_output_b: 12,
            },
            Day1Calibration {
                input: "5a2".into(),
                expected_output_a: 52,
                expected_output_b: 52,
            },
            Day1Calibration {
                input: "a9b54c2ag".into(),
                expected_output_a: 92,
                expected_output_b: 92,
            },
            Day1Calibration {
                input: "12
34"
                .into(),
                expected_output_a: 46,
                expected_output_b: 46,
            },
            Day1Calibration {
                input: "a12b
z3e4fg"
                    .into(),
                expected_output_a: 46,
                expected_output_b: 46,
            },
            Day1Calibration {
                input: "a12b
z3e4fg
5a2b3g1u"
                    .into(),
                expected_output_a: 97,
                expected_output_b: 97,
            },
            Day1Calibration {
                input: "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
                    .into(),
                expected_output_a: 142,
                expected_output_b: 142,
            },
            Day1Calibration {
                input: "1two".into(),
                expected_output_a: 11,
                expected_output_b: 12,
            },
            Day1Calibration {
                input: "three5two".into(),
                expected_output_a: 55,
                expected_output_b: 32,
            },
            Day1Calibration {
                input: "xthree5twoc".into(),
                expected_output_a: 55,
                expected_output_b: 32,
            },
            Day1Calibration {
                input: "xthrebethree5twoc".into(),
                expected_output_a: 55,
                expected_output_b: 32,
            },
            Day1Calibration {
                input: "two1nine
eightwo8three
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                    .into(),
                expected_output_a: 297,
                expected_output_b: 281,
            },
            Day1Calibration {
                input: "1eightwo".into(),
                expected_output_a: 11,
                expected_output_b: 12,
            },
        ];

        for test_case in test_cases {
            let output = puzzle_a(&test_case.input).expect("a calculation failed");
            assert_eq!(
                output, test_case.expected_output_a,
                "input: {:?}",
                test_case.input
            );
            let output = puzzle_b(&test_case.input).expect("b calculation failed");
            assert_eq!(
                output, test_case.expected_output_b,
                "input: {:?}",
                test_case.input
            );
        }
    }
}
