use itertools::Itertools;

const BASE13: [usize; 6] = [1, 13, 169, 2197, 28561, 371293];

#[derive(Debug)]
struct Play {
    hand: [char; 5],
    bid: usize,
}

impl Play {
    fn score(&self) -> usize {
        let mut score = 0;

        for i in 0..5 {
            let base = BASE13[i];
            let card_num = card_to_num(self.hand[i]);
            score += base * card_num;
        }

        score + BASE13[5] * self.hand_type_score()
    }

    fn hand_type_score(&self) -> usize {
        const HIGH_CARD: usize = 0;
        const ONE_PAIR: usize = 1;
        const TWO_PAIR: usize = 2;
        const THREE_OF_A_KIND: usize = 3;
        const FULL_HOUSE: usize = 4;
        const FOUR_OF_A_KIND: usize = 5;
        const FIVE_OF_A_KIND: usize = 6;

        let mut card_counts = vec![0; 13];

        self.hand.iter().for_each(|&card| {
            card_counts[card_to_num(card)] += 1;
        });

        let matches = card_counts
            .into_iter()
            .filter(|&c| c != 0)
            .sorted()
            .collect::<Vec<_>>();

        match matches.len() {
            1 => FIVE_OF_A_KIND,
            2 => {
                if matches[0] == 1 {
                    FOUR_OF_A_KIND
                } else {
                    FULL_HOUSE
                }
            }
            3 => {
                if matches[matches.len() - 1] == 2 {
                    TWO_PAIR
                } else {
                    THREE_OF_A_KIND
                }
            }
            4 => ONE_PAIR,
            5 => HIGH_CARD,
            _ => panic!(),
        }
    }
}

fn card_to_num(card: char) -> usize {
    match card {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!(),
    }
}

fn main() {
    let input = include_str!("../../input.txt");

    let plays = input
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect_tuple::<(_, _)>().unwrap();
            Play {
                hand: split
                    .0
                    .chars()
                    .rev()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                bid: split.1.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let scored_plays = plays
        .iter()
        .map(|play| (play.score(), play))
        .collect::<Vec<_>>();

    let sorted_plays = scored_plays
        .iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, play)| *play)
        .collect::<Vec<_>>();

    let total = sorted_plays
        .iter()
        .enumerate()
        .map(|(i, play)| (i + 1) * play.bid)
        .sum::<usize>();

    println!("{:#?}", total);
}
