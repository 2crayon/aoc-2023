use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn to_known(springs: &[Spring]) -> Vec<SpringKnown> {
    springs
        .iter()
        .map(|spring| match spring {
            Spring::Operational => SpringKnown::Operational,
            Spring::Damaged => SpringKnown::Damaged,
            Spring::Unknown => unreachable!(),
        })
        .collect_vec()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpringKnown {
    Operational,
    Damaged,
}

fn possible_arrangements(springs: &[Spring]) -> Vec<Vec<SpringKnown>> {
    let unknown_indecies = springs
        .iter()
        .enumerate()
        .filter_map(|(i, spring)| match spring {
            Spring::Unknown => Some(i),
            _ => None,
        })
        .collect_vec();

    let mut perms = vec![];

    for perm in 0..2usize.pow(unknown_indecies.len() as u32) {
        let mut perm = perm;
        let mut perm_vec = vec![];

        for _ in 0..unknown_indecies.len() {
            let spring = match perm % 2 {
                0 => SpringKnown::Operational,
                1 => SpringKnown::Damaged,
                _ => unreachable!(),
            };
            perm_vec.push(spring);
            perm /= 2;
        }

        perms.push(perm_vec);
    }

    let arrangements = perms
        .iter()
        .map(|perm| {
            let mut arrangement = springs.to_owned();

            for (i, &n) in unknown_indecies.iter().enumerate() {
                arrangement[n] = match perm[i] {
                    SpringKnown::Operational => Spring::Operational,
                    SpringKnown::Damaged => Spring::Damaged,
                }
            }

            to_known(&arrangement)
        })
        .collect_vec();

    arrangements
}

fn count_valid_arrangements(springs: &[Spring], seq: &[usize]) -> usize {
    let mut count = 0;

    let possible = possible_arrangements(springs);

    for arrangement in possible {
        if is_valid_arrangement(&arrangement, seq) {
            count += 1;
        }
    }

    count
}

fn is_valid_arrangement(springs: &[SpringKnown], seq: &[usize]) -> bool {
    let mut evaluated_seq = vec![];
    let mut count = 0;

    for spring in springs {
        match *spring {
            SpringKnown::Damaged => count += 1,
            SpringKnown::Operational => {
                if count > 0 {
                    evaluated_seq.push(count);
                    count = 0;
                }
            }
        }
    }
    if count > 0 {
        evaluated_seq.push(count);
    }

    evaluated_seq == seq
}

fn main() {
    let input = include_str!("../../input.txt");

    let records = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_whitespace().collect_tuple::<(_, _)>().unwrap();

            let springs = left
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => panic!(),
                })
                .collect_vec();

            let seq = right
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect_vec();

            (springs, seq)
        })
        .collect_vec();

    let sln = records
        .iter()
        .map(|(springs, seq)| count_valid_arrangements(springs, seq))
        .sum::<usize>();

    println!("{:#?}", sln);
}
