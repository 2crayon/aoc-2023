use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node([char; 3]);

impl Node {
    fn from_str(s: &str) -> Self {
        Self(
            s.chars()
                .collect::<Vec<_>>()
                .try_into()
                .expect("invalid node id"),
        )
    }
}

fn find_line(lines: &[(Node, (Node, Node))], node: Node) -> &(Node, (Node, Node)) {
    lines
        .iter()
        .find(|(n, _)| *n == node)
        .expect("node not found")
}

fn main() {
    let input = include_str!("../../input.txt");

    let instructions = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    let lines = input
        .lines()
        .skip(2)
        .map(|line| {
            let split = line.split(" = ").collect_tuple::<(_, _)>().unwrap();
            let dests = split
                .1
                .chars()
                .filter(|c| c.is_ascii_alphabetic())
                .collect::<String>();
            let dests = dests.split_at(3);

            let node = Node::from_str(split.0);
            let mapped_nodes = (Node::from_str(dests.0), Node::from_str(dests.1));

            (node, mapped_nodes)
        })
        .collect::<Vec<_>>();

    let mut steps = 0;
    let mut current_line = find_line(&lines, Node::from_str("AAA"));

    for instr in instructions.into_iter().cycle() {
        let node = current_line.0;
        let left = current_line.1 .0;
        let right = current_line.1 .1;

        if node == Node::from_str("ZZZ") {
            break;
        }

        match instr {
            Instruction::Left => current_line = find_line(&lines, left),
            Instruction::Right => current_line = find_line(&lines, right),
        }
        steps += 1;
    }

    println!("{:#?}", steps);
}
