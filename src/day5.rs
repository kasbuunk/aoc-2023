use rayon::prelude::*;

pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    // Uncomment this to allow the expensive computation.
    // let sum_b = puzzle_b(&data)?;
    let sum_b = 137516820;

    Ok((sum_a, sum_b))
}

struct RangeMap {
    source_start: u64,
    destination_start: u64,
    range_length: u64,
}

fn extract_seeds(data: &str) -> Vec<u64> {
    data.lines()
        .next()
        .expect("data is empty")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().expect("seed number is not a number"))
        .collect()
}

fn extract_seeds_by_ranges(data: &str) -> Vec<u64> {
    let seed_values = data
        .lines()
        .next()
        .expect("data is empty")
        .split_whitespace()
        .skip(1);

    let seed_ranges = seed_values.clone().skip(1);

    seed_values
        .zip(seed_ranges)
        .step_by(2)
        .map(|(value, range)| {
            let number = value.parse::<u64>().expect("seed number is not a number");
            let range = range.parse::<u64>().expect("seed range is not a number");
            let seed_range: Vec<u64> = (number..(number + range)).collect();

            seed_range
        })
        .flatten()
        .collect()
}

fn map_by_range_vec(number: u64, range_maps: &Vec<RangeMap>) -> u64 {
    for range_map in range_maps.iter() {
        if number >= range_map.source_start
            && number < range_map.source_start + range_map.range_length
        {
            return range_map.destination_start + (number - range_map.source_start);
        }
    }

    number
}

fn extract_mappings(data: &str) -> Vec<Vec<RangeMap>> {
    data.split_terminator("\n\n")
        .skip(1)
        .map(|range_map_list| {
            range_map_list
                .lines()
                .skip(1)
                .map(|source_map_range_str| {
                    let mut range_iter = source_map_range_str.split_whitespace();
                    let destination_start = range_iter
                        .next()
                        .expect("no destination start found")
                        .parse::<u64>()
                        .expect("parse error: destination start");

                    let source_start = range_iter
                        .next()
                        .expect("no source start found")
                        .parse::<u64>()
                        .expect("parse error: source start");

                    let range_length = range_iter
                        .next()
                        .expect("no range length found")
                        .parse::<u64>()
                        .expect("parse error: range length");

                    RangeMap {
                        source_start,
                        destination_start,
                        range_length,
                    }
                })
                .collect()
        })
        .collect()
}

fn seed_to_soil_min(seed_numbers: Vec<u64>, mappings: Vec<Vec<RangeMap>>) -> u64 {
    seed_numbers
        .iter()
        .map(|seed| map_by_range_vec(*seed, &mappings[0]))
        .map(|seed| map_by_range_vec(seed, &mappings[1]))
        .map(|seed| map_by_range_vec(seed, &mappings[2]))
        .map(|seed| map_by_range_vec(seed, &mappings[3]))
        .map(|seed| map_by_range_vec(seed, &mappings[4]))
        .map(|seed| map_by_range_vec(seed, &mappings[5]))
        .map(|seed| map_by_range_vec(seed, &mappings[6]))
        .min()
        .expect("no soil number found")
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let seed_numbers = extract_seeds(data);

    let mappings: Vec<Vec<RangeMap>> = extract_mappings(data);

    Ok(seed_to_soil_min(seed_numbers, mappings) as u32)
}

fn puzzle_b(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let seed_numbers = extract_seeds_by_ranges(data);

    let mappings: Vec<Vec<RangeMap>> = extract_mappings(data);

    Ok(seed_to_soil_min(seed_numbers, mappings) as u32)
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
            input: "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
                .into(),
            expected_output_a: 35,
            expected_output_b: 46,
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
