pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    Ok((sum_a, sum_b))
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let width = data
        .lines()
        .next()
        .expect("data does not contain a single line")
        .len();

    let empty_line = ".".repeat(width);

    use std::iter::once;

    // Prepend and append the empty line for easier processing.
    let lines: Vec<&str> = once(empty_line.as_str())
        .chain(data.split_terminator('\n'))
        .chain(once(empty_line.as_str()))
        .collect();

    let sum: u32 = lines
        .windows(3)
        .map(|windows| {
            let (upper_line, current_line, lower_line) = (windows[0], windows[1], windows[2]);

            let numbers_found = find_numbers(current_line);

            numbers_found
                .iter()
                .filter(|(x, y)| {
                    let check_symbol_positions_start = if *y <= 0 { 0 } else { *y - 1 };
                    let check_symbol_positions_end =
                        std::cmp::min(width as u32 - 1, x.to_string().len() as u32 + *y);

                    for index in check_symbol_positions_start..=check_symbol_positions_end {
                        if let Some(character) = upper_line.chars().nth(index as usize) {
                            if !character.is_numeric() && character != '.' {
                                return true;
                            }
                        }
                        if let Some(character) = current_line.chars().nth(index as usize) {
                            if !character.is_numeric() && character != '.' {
                                return true;
                            }
                        }
                        if let Some(character) = lower_line.chars().nth(index as usize) {
                            if !character.is_numeric() && character != '.' {
                                return true;
                            }
                        }
                    }

                    false
                })
                .map(|&(number, _)| number)
                .collect::<Vec<u32>>()
        })
        .flat_map(|x| x)
        .sum();

    Ok(sum)
}

fn puzzle_b(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let width = data
        .lines()
        .next()
        .expect("data does not contain a single line")
        .len();

    let empty_line = ".".repeat(width);

    use std::iter::once;

    // Prepend and append the empty line for easier processing.
    let lines: Vec<&str> = once(empty_line.as_str())
        .chain(data.split_terminator('\n'))
        .chain(once(empty_line.as_str()))
        .collect();

    let sum: u32 = lines
        .windows(3)
        .map(|windows| {
            let (upper_line, current_line, lower_line) = (windows[0], windows[1], windows[2]);

            let gears_found = find_gears(current_line);

            let numbers_upper = find_numbers(upper_line);
            let numbers_current = find_numbers(current_line);
            let numbers_lower = find_numbers(lower_line);
            let number_lines = vec![numbers_upper, numbers_current, numbers_lower];

            gears_found
                .iter()
                .filter_map(|gear_position| {
                    // Iterate over the numbers found. For each number, check if the gear is within
                    // range. If so, the number is collected. If the collected amount of numbers is
                    // 2, then return Some(first*second)
                    let mut numbers_in_range: Vec<u32> = vec![];

                    let _ = number_lines.iter().for_each(|number_line| {
                        let _ = number_line.iter().for_each(|(number, position)| {
                            // Determine the range the gear must be in.
                            let check_symbol_positions_start =
                                if *position <= 0 { 0 } else { *position - 1 };

                            let check_symbol_positions_end = std::cmp::min(
                                width as u32 - 1,
                                number.to_string().len() as u32 + *position,
                            );

                            if *gear_position >= check_symbol_positions_start
                                && *gear_position <= check_symbol_positions_end
                            {
                                numbers_in_range.push(*number);
                            }
                        });
                    });

                    match numbers_in_range.len() {
                        2 => Some(numbers_in_range[0] * numbers_in_range[1]),
                        _ => None,
                    }
                })
                .collect::<Vec<u32>>()
        })
        .flat_map(|x| x)
        .sum();

    Ok(sum)
}

fn find_gears(line: &str) -> Vec<u32> {
    line.chars()
        .enumerate()
        .filter(|(_, symbol)| *symbol == '*')
        .map(|(position, _)| position as u32)
        .collect()
}

fn find_numbers(_line: &str) -> Vec<(u32, u32)> {
    let mut numbers = vec![];

    let mut number_found = false;
    let mut digits = vec![];
    let mut position_first_digit = 0;

    for (index, character) in _line.chars().enumerate() {
        match number_found {
            true => match character.is_numeric() {
                true => digits.push(
                    character
                        .to_digit(10)
                        .expect("char is numeric but cannot be converted to digit"),
                ),
                false => {
                    let n = digits_to_number(digits);
                    numbers.push((n, position_first_digit as u32));

                    // Calculate a number found from the digits.
                    digits = vec![];
                    number_found = false;
                }
            },
            false => match character.is_numeric() {
                true => {
                    number_found = true;
                    position_first_digit = index;
                    digits.push(
                        character
                            .to_digit(10)
                            .expect("char is numeric but cannot be converted to digit"),
                    );
                }
                false => {}
            },
        }
    }

    if number_found {
        let n = digits_to_number(digits);
        numbers.push((n, position_first_digit as u32));
    }

    numbers
}

fn digits_to_number(digits: Vec<u32>) -> u32 {
    let amount_of_digits = digits.len();

    let mut n = 0;

    for digit_index in 0..amount_of_digits {
        n = n + digits[digit_index] * 10_u32.pow(amount_of_digits as u32 - digit_index as u32 - 1);
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        input: &'static str,
        expected_output_a: u32,
        expected_output_b: u32,
    }

    #[test]
    fn puzzle() {
        let test_cases = vec![
            TestCase {
                input: "..12=..",
                expected_output_a: 12,
                expected_output_b: 0,
            },
            TestCase {
                input: "..*34",
                expected_output_a: 34,
                expected_output_b: 0,
            },
            TestCase {
                input: "123..321
...*..2&",
                expected_output_a: 446,
                expected_output_b: 0,
            },
            TestCase {
                input: "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
                expected_output_a: 4361,
                expected_output_b: 467835,
            },
        ];

        for test_case in test_cases {
            let output = puzzle_a(test_case.input).expect("calculation a failed");
            assert_eq!(
                output, test_case.expected_output_a,
                "input: {:?}",
                test_case.input
            );
            let output = puzzle_b(test_case.input).expect("calculation b failed");
            assert_eq!(
                output, test_case.expected_output_b,
                "input: {:?}",
                test_case.input
            );
        }
    }
}
