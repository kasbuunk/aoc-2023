use std::collections::HashMap;
use std::fmt;
use std::time::Instant;

pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    println!("day 8 b: {}", sum_b);

    Ok((sum_a, sum_b as u32))
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let node_start = must_parse_node_id("AAA");
    let node_needle = must_parse_node_id("ZZZ");
    let mut map = extract_map(data);

    Ok(map.traverse_graph(&node_start, &node_needle) as u32)
}

fn puzzle_b(data: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let mut map = extract_map(data);

    let nodes_start = extract_starting_nodes(&map.nodes);

    Ok(map.traverse_graph_simultaneous(nodes_start) as u64)
}

struct Map {
    directions: DirectionList,
    nodes: DirectedGraph,
}

impl Map {
    fn traverse_graph(&mut self, node_start: &NodeId, node_needle: &NodeId) -> usize {
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

    fn traverse_graph_simultaneous(&mut self, nodes_start: Vec<NodeId>) -> usize {
        let now = Instant::now();
        let mut counter: u64 = 0;
        let base: u64 = 2;
        let mut power_of_two: u32 = 0;
        let mut cursors = nodes_start.clone();

        println!("Nodes at {}: {}", counter, format_nodes(&nodes_start));

        self.directions
            .position(|direction| {
                cursors = cursors
                    .iter()
                    .map(|node| (*traverse(&self.nodes, node, &direction)).clone())
                    .collect();

                counter += 1;
                if counter >= base.pow(power_of_two) {
                    println!(
                        "Index {} after {}s: {}",
                        counter,
                        now.elapsed().as_secs(),
                        format_nodes(&cursors)
                    );
                    power_of_two += 1;
                }

                cursors.iter().all(|node| node.0[2] == 'Z')
            })
            .expect("could not find the desired node")
            + 1 // The desired position is 1-indexed.
    }
}

fn format_nodes(nodes: &[NodeId]) -> String {
    let formatted_ids = nodes
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    formatted_ids
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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct NodeId([char; 3]);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

impl From<Vec<char>> for NodeId {
    fn from(chars: Vec<char>) -> NodeId {
        NodeId(
            chars
                .into_iter()
                .take(3)
                .collect::<Vec<_>>()
                .try_into()
                .expect("could not convert chars to node id"),
        )
    }
}

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

fn extract_starting_nodes(graph: &DirectedGraph) -> Vec<NodeId> {
    graph
        .iter()
        .filter_map(|(node, _)| match (*node).0[2] == 'A' {
            true => Some((*node).clone()),
            false => None,
        })
        .collect()
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
        expected_output_b: u64,
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
                expected_output_b: 2,
            },
            TestCase {
                input: "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
                    .into(),
                expected_output_a: 6,
                expected_output_b: 6,
            },
            TestCase {
                input: "LR

AAA = (11B, XXX)
11B = (XXX, ZZZ)
ZZZ = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
                    .into(),
                expected_output_a: 2,
                expected_output_b: 6,
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
