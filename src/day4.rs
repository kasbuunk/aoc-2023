use std::collections::HashSet;

pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    Ok((sum_a, sum_b))
}

type CardNumber = u32;

struct ScratchCard {
    _card_number: CardNumber,
    winning_numbers: HashSet<u32>,
    drawn_numbers: HashSet<u32>,
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let sum: u32 = data
        .lines()
        .map(|line| parse_scratch_card(line))
        .map(|scratch_card| calculate_winning_matches(&scratch_card))
        .map(|winning_matches| calculate_points(winning_matches))
        .sum();

    Ok(sum)
}

fn parse_scratch_card(line: &str) -> ScratchCard {
    let card_prefix = "Card ";
    let mut parser = line[card_prefix.len()..].split_terminator(": ");
    let card_number: u32 = parser
        .next()
        .expect("parse error: no colon in line")
        .trim()
        .parse()
        .expect("parse error: id is not a u32");

    let mut numbers_parser = parser
        .next()
        .expect("parse error: no winning numbers")
        .trim()
        .split_terminator(" | ");

    let winning_numbers_str = numbers_parser
        .next()
        .expect("parse error: no vertical bar found")
        .trim();
    let winning_numbers_list = string_to_num_list(winning_numbers_str);

    let mut winning_numbers = HashSet::new();
    winning_numbers_list.iter().for_each(|number| {
        let _ = winning_numbers.insert(*number);
    });

    let drawn_numbers_str = numbers_parser
        .next()
        .expect("parse error: no drawn numbers found")
        .trim();
    let drawn_numbers_list = string_to_num_list(drawn_numbers_str);

    let mut drawn_numbers = HashSet::new();
    drawn_numbers_list.iter().for_each(|number| {
        let _ = drawn_numbers.insert(*number);
    });

    ScratchCard {
        _card_number: card_number,
        winning_numbers,
        drawn_numbers,
    }
}

fn string_to_num_list(number_string: &str) -> Vec<u32> {
    number_string
        .split_whitespace()
        .map(|n| {
            n.parse::<u32>()
                .expect("parse error: non-u32 char in number list")
        })
        .collect()
}

fn calculate_winning_matches(card: &ScratchCard) -> u32 {
    card.winning_numbers
        .iter()
        .filter(|winning_number| card.drawn_numbers.contains(winning_number))
        .count() as u32
}

fn calculate_points(winning_matches: u32) -> u32 {
    if winning_matches == 0 {
        return 0;
    }
    let base: u32 = 2;

    base.pow(winning_matches - 1)
}

fn puzzle_b(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut card_copies: Vec<u32> = data.lines().map(|_| 1).collect();

    let sum: u32 = data
        .lines()
        .map(|line| parse_scratch_card(line))
        .enumerate()
        .map(|(index, scratch_card)| {
            let copies = card_copies[index];
            let to_add = calculate_winning_matches(&scratch_card);
            for j in 1..=to_add {
                match card_copies.iter().nth(index + j as usize) {
                    Some(_) => {
                        card_copies[index + j as usize] = card_copies[index + j as usize] + copies;
                    }
                    None => break,
                }
            }

            copies
        })
        .sum();

    Ok(sum)
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
        let test_cases = vec![
            TestCase {
                input: "".into(),
                expected_output_a: 0,
                expected_output_b: 0,
            },
            TestCase {
                input: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".into(),
                expected_output_a: 8,
                expected_output_b: 1,
            },
            TestCase {
                input: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
                    .into(),
                expected_output_a: 13,
                expected_output_b: 30,
            },
        ];
        for test_case in test_cases {
            let output = puzzle_a(&test_case.input).expect("could not solve puzzle a");
            assert_eq!(
                output, test_case.expected_output_a,
                "input: {}",
                test_case.input
            );
            let output = puzzle_b(&test_case.input).expect("could not solve puzzle b");
            assert_eq!(
                output, test_case.expected_output_b,
                "input: {}",
                test_case.input
            );
        }
    }
}
