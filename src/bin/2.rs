use itertools::Itertools;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn main() {
    let input = include_str!("../../input.txt");

    let lines = input.lines().enumerate().map(|(i, line)| (i + 1, line));

    let mut sum = 0;

    for (game_id, line) in lines {
        let pad = line.find(": ").unwrap() + 2;
        let line = &line[pad..];

        let sets = line.split("; ");

        let mut game_is_impossible = false;
        for set in sets {
            if is_impossible(set) {
                game_is_impossible = true;
                break;
            }
        }

        if !game_is_impossible {
            sum += game_id;
        }
    }

    println!("{:?}", sum);
}

fn is_impossible(set: &str) -> bool {
    let batches = set.split(", ");
    for batch in batches {
        let (amount, color) = batch.split(" ").collect_tuple::<(_, _)>().unwrap();

        let amount = amount.parse::<i32>().unwrap();

        if color == "red" && amount > RED
            || color == "green" && amount > GREEN
            || color == "blue" && amount > BLUE
        {
            return true;
        }
    }

    return false;
}
