use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let pad = input.find(':').unwrap() + 2;
    let cards = input
        .lines()
        .map(|line| &line[pad..])
        .map(|line| {
            line.split('|')
                .map(|side| {
                    side.split(" ")
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                })
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let scores = cards.iter().map(|card| score(card)).collect::<Vec<_>>();

    println!("{:#?}", scores.iter().sum::<i32>());
}

fn score((win_nums, rolled_nums): &(Vec<&str>, Vec<&str>)) -> i32 {
    let mut score = 0i32;

    for &n in win_nums {
        for &r in rolled_nums {
            if n == r {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
                break;
            }
        }
    }

    return score;
}
