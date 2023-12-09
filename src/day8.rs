pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    Ok((sum_a, sum_b))
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let node_start = must_parse_node_id("AAA");
    let node_needle = must_parse_node_id("ZZZ");
    let mut map = extract_map(data, node_start);

    Ok(map.find_node(node_needle) as u32)
}

fn puzzle_b(_data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(0)
}

struct Map {
    directions: DirectionList,
    nodes: DirectedGraph,
}

impl Map {
    fn find_node(&mut self, node_needle: NodeId) -> usize {
        self.directions
            .position(|direction| {
                let node = self.nodes.traverse(&direction);
                if node.id == node_needle {
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

struct Node {
    id: NodeId,
    left_edge: NodeId,
    right_edge: NodeId,
}

struct DirectedGraph {
    nodes: Vec<Node>,
    cursor: usize,
}

impl DirectedGraph {
    fn new(nodes: Vec<Node>, node_start: NodeId) -> Self {
        let (cursor, _) = Self::find_position(&nodes, node_start);
        DirectedGraph { nodes, cursor }
    }

    fn find_position(nodes: &[Node], node_id_needle: NodeId) -> (usize, &Node) {
        nodes
            .iter()
            .enumerate()
            .find(|(_index, node)| node.id == node_id_needle)
            .expect("failed to traverse node graph")
    }

    fn traverse(&mut self, direction: &Direction) -> &Node {
        let node = &self.nodes[self.cursor];
        let new_node_id = match direction {
            Direction::Left => node.left_edge,
            Direction::Right => node.right_edge,
        };

        let (new_node_index, new_node) = Self::find_position(&self.nodes, new_node_id);

        self.cursor = new_node_index;

        new_node
    }
}

fn extract_map(data: &str, node_start: NodeId) -> Map {
    let directions = extract_directions(data.lines().next().expect("could not find first line"));
    let graph = extract_graph(data, node_start);
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

fn extract_graph(data: &str, node_start: NodeId) -> DirectedGraph {
    let nodes = data
        .lines()
        .skip(2)
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let node_id = must_parse_node_id(line_iter.next().expect("no first node found"));

            let mut line_iter = line_iter.skip(1);

            let left_node_id_str = line_iter.next().expect("no left node found");
            let left_node_id = must_parse_node_id(&left_node_id_str[1..=3]);

            let right_node_id_str = line_iter.next().expect("no right node found");
            let right_node_id = must_parse_node_id(&right_node_id_str[..=2]);

            Node {
                id: node_id,
                left_edge: left_node_id,
                right_edge: right_node_id,
            }
        })
        .collect();
    DirectedGraph::new(nodes, node_start)
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
