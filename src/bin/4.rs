use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Card {
    id: usize,
    score: usize,
}

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

    let mut cards = cards
        .iter()
        .enumerate()
        .map(|(i, card)| Card {
            id: i,
            score: matches(card),
        })
        .collect::<Vec<_>>();

    let original_cards = cards.clone();

    for i in 0..original_cards.len() {
        let count = cards
            .iter()
            .filter(|card| card.id == i)
            .collect::<Vec<_>>()
            .len();
        let score = original_cards[i].score;

        for j in 1..=score {
            for _ in 0..count {
                cards.push(original_cards[i + j]);
            }
        }
    }

    println!("{}", cards.len());
}

fn matches((win_nums, rolled_nums): &(Vec<&str>, Vec<&str>)) -> usize {
    let mut matches = 0;

    for &n in win_nums {
        for &r in rolled_nums {
            if n == r {
                matches += 1;
                break;
            }
        }
    }

    return matches;
}
