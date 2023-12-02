use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");

    let lines = input.lines().enumerate().map(|(i, line)| (i + 1, line));

    let mut sum = 0;

    for (_, line) in lines {
        let pad = line.find(": ").unwrap() + 2;
        let line = &line[pad..];

        let sets = line.split("; ");

        let mut most_red = 0;
        let mut most_green = 0;
        let mut most_blue = 0;

        for set in sets {
            let groups = set.split(", ");
            let groups = groups.map(|group| {
                let (amount, color) = group.split(" ").collect_tuple::<(_, _)>().unwrap();
                return (amount.parse::<usize>().unwrap(), color);
            });
            for (amount, color) in groups {
                match color {
                    "red" => most_red = most_red.max(amount),
                    "green" => most_green = most_green.max(amount),
                    "blue" => most_blue = most_blue.max(amount),
                    _ => panic!(),
                }
            }
        }

        sum += most_red * most_green * most_blue;
    }

    println!("{}", sum);
}
