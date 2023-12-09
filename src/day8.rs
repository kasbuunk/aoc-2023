use std::collections::HashMap;

pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    Ok((sum_a, sum_b))
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let node_start = must_parse_node_id("AAA");
    let node_needle = must_parse_node_id("ZZZ");
    let mut map = extract_map(data);

    Ok(map.find_node(&node_start, &node_needle) as u32)
}

fn puzzle_b(_data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(0)
}

struct Map {
    directions: DirectionList,
    nodes: DirectedGraph,
}

impl Map {
    fn find_node(&mut self, node_start: &NodeId, node_needle: &NodeId) -> usize {
        let mut cursor = node_start;
        self.directions
            .position(|direction| {
                cursor = traverse(&self.nodes, &cursor, &direction);
                if cursor == node_needle {
                    true
                } else {
                    false
                }
            })
            .expect("could not find the desired node")
            + 1 // The desired position is 1-indexed.
    }
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

struct DirectionList {
    directions: Vec<Direction>,
    cursor: usize,
}

impl DirectionList {
    fn new(directions: Vec<Direction>) -> Self {
        DirectionList {
            directions,
            cursor: 0,
        }
    }
}

impl Iterator for DirectionList {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        let direction = self.directions[self.cursor].clone();
        self.cursor = (self.cursor + 1) % self.directions.len();

        Some(direction)
    }
}

type NodeId = [char; 3];

type DirectedGraph = HashMap<NodeId, NodeDirectory>;

struct NodeDirectory {
    left: NodeId,
    right: NodeId,
}

fn traverse<'a>(
    graph: &'a DirectedGraph,
    node_start: &NodeId,
    direction: &Direction,
) -> &'a NodeId {
    let node = graph.get(node_start).expect("node id not found");
    match direction {
        Direction::Left => &node.left,
        Direction::Right => &node.right,
    }
}

fn extract_map(data: &str) -> Map {
    let directions = extract_directions(data.lines().next().expect("could not find first line"));
    let graph = extract_graph(data);
    Map {
        directions,
        nodes: graph,
    }
}

fn extract_directions(directions_str: &str) -> DirectionList {
    let directions = directions_str
        .trim()
        .chars()
        .map(|direction_character| match direction_character {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("encountered unexpected non 'L/R' direction character"),
        })
        .collect();

    DirectionList::new(directions)
}

fn extract_graph(data: &str) -> DirectedGraph {
    let mut graph: DirectedGraph = DirectedGraph::new();

    data.lines().skip(2).for_each(|line| {
        let mut line_iter = line.split_whitespace();
        let node_id = must_parse_node_id(line_iter.next().expect("no first node found"));

        let mut line_iter = line_iter.skip(1);

        let left_node_id_str = line_iter.next().expect("no left node found");
        let left_node_id = must_parse_node_id(&left_node_id_str[1..=3]);

        let right_node_id_str = line_iter.next().expect("no right node found");
        let right_node_id = must_parse_node_id(&right_node_id_str[..=2]);

        let _ = graph.insert(
            node_id,
            NodeDirectory {
                left: left_node_id,
                right: right_node_id,
            },
        );
    });

    graph
}

fn must_parse_node_id(node_id_string: &str) -> NodeId {
    node_id_string
        .chars()
        .take(3)
        .collect::<Vec<char>>()
        .try_into()
        .expect("could not convert data to node id")
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
                input: "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
                    .into(),
                expected_output_a: 2,
                expected_output_b: 0,
            },
            TestCase {
                input: "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
                    .into(),
                expected_output_a: 6,
                expected_output_b: 0,
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
