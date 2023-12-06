use itertools::Itertools;

#[derive(Debug)]
struct Race {
    time: u32,
    record: u32,
}

impl Race {
    fn is_beating_record(&self, hold_time: u32) -> bool {
        let time_left = self.time - hold_time;
        let speed = hold_time;
        let distance = speed * time_left;

        distance > self.record
    }
}

fn main() {
    let input = include_str!("../../input.txt");

    let a = input
        .lines()
        .map(|line| {
            (&line[9..])
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect_tuple::<(_, _)>()
        .unwrap();

    let mut races = vec![];

    for i in 0..a.0.len() {
        let new_race = Race {
            time: a.0[i],
            record: a.1[i],
        };
        races.push(new_race);
    }

    let mut all_ways = vec![];

    for race in &races {
        let mut ways_to_win = 0;

        for i in 0..=race.time {
            if race.is_beating_record(i) {
                ways_to_win += 1;
            } else {
                continue;
            }
        }

        all_ways.push(ways_to_win);
    }

    let solution = all_ways.iter().product::<i32>();

    println!("{:#?}", solution);
}
