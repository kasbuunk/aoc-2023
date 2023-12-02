pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    Ok((sum_a, sum_b))
}

enum Colour {
    Red,
    Green,
    Blue,
}

struct Cube {
    colour: Colour,
    amount: u32,
}

struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

struct GameRecord {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let sum = data
        .lines()
        .map(|line| {
            let line_prefix = "Game ";
            assert!(line.starts_with(line_prefix));

            let mut colon_split = line[line_prefix.len()..].split_terminator(':');

            let id: u32 = colon_split
                .next()
                .expect("no colon found")
                .parse()
                .expect("string slice cannot be parsed as integer");

            let cube_sets_str = colon_split
                .next()
                .expect("no game record after colon")
                .trim();

            let cube_sets = cube_sets_str
                .split_terminator(';')
                .map(|cube_set| {
                    let cubes = cube_set.split_terminator(',').map(|cube| {
                        let mut cubes = cube.split_whitespace();
                        let amount: u32 = cubes
                            .next()
                            .expect("no number found in cube")
                            .parse()
                            .expect("number could not be parsed");

                        let colour: Colour = match cubes.next().expect("no number found in cube") {
                            "red" => Colour::Red,
                            "green" => Colour::Green,
                            "blue" => Colour::Blue,
                            _ => panic!("cannot parse cube colour"),
                        };

                        Cube { amount, colour }
                    });

                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    for cube in cubes {
                        match cube.colour {
                            Colour::Red => red = red + cube.amount,
                            Colour::Green => green = green + cube.amount,
                            Colour::Blue => blue = blue + cube.amount,
                        }
                    }

                    CubeSet { red, green, blue }
                })
                .collect();

            GameRecord { id, cube_sets }
        })
        .filter(|game_record| {
            // The record is included if there is no cube set with excessive cubes of a particular
            // colour.
            !game_record
                .cube_sets
                .iter()
                .any(|cube_set| cube_set.red > 12 || cube_set.green > 13 || cube_set.blue > 14)
        })
        .map(|game_record| game_record.id)
        .sum();

    Ok(sum)
}

fn puzzle_b(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let sum = data
        .lines()
        .map(|line| {
            let line_prefix = "Game ";
            assert!(line.starts_with(line_prefix));

            let mut colon_split = line[line_prefix.len()..].split_terminator(':');

            let id: u32 = colon_split
                .next()
                .expect("no colon found")
                .parse()
                .expect("string slice cannot be parsed as integer");

            let cube_sets_str = colon_split
                .next()
                .expect("no game record after colon")
                .trim();

            let cube_sets = cube_sets_str
                .split_terminator(';')
                .map(|cube_set| {
                    let cubes = cube_set.split_terminator(',').map(|cube| {
                        let mut cubes = cube.split_whitespace();
                        let amount: u32 = cubes
                            .next()
                            .expect("no number found in cube")
                            .parse()
                            .expect("number could not be parsed");

                        let colour: Colour = match cubes.next().expect("no number found in cube") {
                            "red" => Colour::Red,
                            "green" => Colour::Green,
                            "blue" => Colour::Blue,
                            _ => panic!("cannot parse cube colour"),
                        };

                        Cube { amount, colour }
                    });

                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    for cube in cubes {
                        match cube.colour {
                            Colour::Red => red = red + cube.amount,
                            Colour::Green => green = green + cube.amount,
                            Colour::Blue => blue = blue + cube.amount,
                        }
                    }

                    CubeSet { red, green, blue }
                })
                .collect();

            GameRecord { id, cube_sets }
        })
        .map(|_game_record| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cube_set in _game_record.cube_sets.iter() {
                if cube_set.red > red {
                    red = cube_set.red
                }
                if cube_set.green > green {
                    green = cube_set.green
                }
                if cube_set.blue > blue {
                    blue = cube_set.blue
                }
            }

            CubeSet { red, green, blue }
        })
        .map(|cube_set| cube_set.red * cube_set.green * cube_set.blue)
        .sum();

    Ok(sum)
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
    fn test_puzzle_a() {
        let test_cases = vec![
            TestCase {
                input: "",
                expected_output_a: 0,
                expected_output_b: 0,
            },
            TestCase {
                input: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                expected_output_a: 1,
                expected_output_b: 4 * 2 * 6,
            },
            TestCase {
                input: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                expected_output_a: 8,
                expected_output_b: 2286,
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
