use std::collections::HashMap;
pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    Ok((sum_a, sum_b))
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let include_joker = false;
    play_poker(data, include_joker)
}

fn puzzle_b(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let include_joker = true;
    play_poker(data, include_joker)
}

fn play_poker(data: &str, include_joker: bool) -> Result<u32, Box<dyn std::error::Error>> {
    let mut plays = extract_plays(data, include_joker)?;

    plays.sort_by(|play_a, play_b| {
        let hand_type_a = derive_hand_type(&play_a.hand);
        let hand_type_b = derive_hand_type(&play_b.hand);
        match hand_type_a.cmp(&hand_type_b) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            _ => play_a.hand.cmp(&play_b.hand),
        }
    });

    let sum = plays
        .iter()
        .enumerate()
        .map(|(index, value)| (index as u32 + 1) * value.bet)
        .sum();

    Ok(sum)
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct Play {
    hand: Vec<Card>,
    bet: u32,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn derive_hand_type(hand: &[Card]) -> HandType {
    let mut jokers = 0;
    let mut hand_map: HashMap<&Card, u8> = HashMap::new();

    hand.iter().for_each(|card| {
        if *card == Card::Joker {
            jokers += 1;
        } else {
            let card_amount = hand_map.entry(card).or_insert(0);
            *card_amount += 1;
        }
    });

    if jokers == 5 {
        return HandType::FiveOfAKind;
    }

    let mut highest_amount: u8 = 0;
    let mut second_highest_amount: u8 = 0;

    hand_map.iter().for_each(|(_card, cards_amount)| {
        if cards_amount > &highest_amount {
            second_highest_amount = highest_amount;
            highest_amount = *cards_amount;
        } else if cards_amount > &second_highest_amount {
            second_highest_amount = *cards_amount;
        }
    });

    let hand_type = match highest_amount {
        5 => HandType::FiveOfAKind,
        4 => match jokers {
            0 => HandType::FourOfAKind,
            _ => HandType::FiveOfAKind,
        },
        3 => match jokers {
            0 => match second_highest_amount {
                0 => panic!("with 3 as highest_amount, cannot have 0 second_highest_amount"),
                1 => HandType::ThreeOfAKind,
                2 => HandType::FullHouse,
                _ => panic!("second_highest_amount cannot be greater than highest amount"),
            },
            1 => HandType::FourOfAKind,
            _ => HandType::FiveOfAKind,
        },
        2 => match jokers {
            0 => match second_highest_amount {
                0 => panic!("with 3 as highest_amount, cannot have 0 second_highest_amount"),
                1 => HandType::OnePair,
                2 => HandType::TwoPair,
                _ => panic!("second_highest_amount cannot be greater than highest amount"),
            },
            1 => match second_highest_amount {
                0 => panic!("with 3 as highest_amount, cannot have 0 second_highest_amount"),
                1 => HandType::ThreeOfAKind,
                2 => HandType::FullHouse,
                _ => panic!("second_highest_amount cannot be greater than highest amount"),
            },
            2 => HandType::FourOfAKind,
            _ => HandType::FiveOfAKind,
        },
        1 => match jokers {
            0 => HandType::HighCard,
            1 => HandType::OnePair,
            2 => HandType::ThreeOfAKind,
            3 => HandType::FourOfAKind,
            _ => HandType::FiveOfAKind,
        },
        _ => HandType::HighCard,
    };

    hand_type
}

fn card_from_char(character: char, j_is_joker: bool) -> Card {
    match character {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => match j_is_joker {
            true => Card::Joker,
            false => Card::Jack,
        },
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("parse error: card could not be parsed"),
    }
}

fn extract_plays(data: &str, j_is_joker: bool) -> Result<Vec<Play>, Box<dyn std::error::Error>> {
    let plays = data
        .trim()
        .lines()
        .map(|play| {
            let mut play_iter = play.trim().split_whitespace();

            if let (Some(hand_str), Some(bet_str)) = (play_iter.next(), play_iter.next()) {
                let hand = hand_str
                    .chars()
                    .map(|card_character| card_from_char(card_character, j_is_joker))
                    .collect();

                let bet = bet_str.parse::<u32>().expect("bet amount is not a number");

                Play { hand, bet }
            } else {
                panic!("parse error: play could not be parsed");
            }
        })
        .collect();

    Ok(plays)
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
                input: "33456 10".into(),
                expected_output_a: 10,
                expected_output_b: 10,
            },
            TestCase {
                input: "AA456 10
            55567 20"
                    .into(),
                expected_output_a: 50,
                expected_output_b: 50,
            },
            TestCase {
                input: "33456 1
            KKJ77 300
            55567 20"
                    .into(),
                expected_output_a: 661,
                expected_output_b: 941,
            },
            TestCase {
                input: "KTJJT 20
            QQQJA 300"
                    .into(),
                expected_output_a: 620,
                expected_output_b: 340,
            },
            TestCase {
                input: "T55J5 1
KTJJT 20
QQQJA 300"
                    .into(),
                expected_output_a: 922,
                expected_output_b: 661,
            },
            TestCase {
                input: "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
                    .into(),
                expected_output_a: 6440,
                expected_output_b: 5905,
            },
        ];

        for test_case in test_cases {
            let output = puzzle_a(&test_case.input).expect("solving puzzle a");
            assert_eq!(
                output, test_case.expected_output_a,
                "puzzle a failed for input: {}",
                test_case.input,
            );

            let output = puzzle_b(&test_case.input).expect("solving puzzle b");
            assert_eq!(
                output, test_case.expected_output_b,
                "puzzle b failed for input: {}",
                test_case.input,
            );
        }
    }
}
