use itertools::Itertools;

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn is_beating_record(&self, hold_time: u64) -> bool {
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
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple::<(_, _)>()
        .unwrap();

    let race = Race {
        time: a.0,
        record: a.1,
    };

    let mut ways_to_win = 0;

    for i in 0..=race.time {
        if race.is_beating_record(i) {
            ways_to_win += 1;
        } else {
            continue;
        }
    }

    println!("{:#?}", ways_to_win);
}
