fn calibration_sum(input: &str) -> u32 {
    let digit_lines = input
        .lines()
        .map(|line| {
            let mut digit_line = String::new();
            let mut letter_sequence = Vec::new();

            for c in line.chars() {
                if c.is_ascii_digit() {
                    digit_line.push(c);
                    letter_sequence.clear();
                } else {
                    letter_sequence.push(c);
                }

                for combination in combinations(&letter_sequence) {
                    match digitized(&combination) {
                        Some(c) => digit_line.push(c),
                        None => continue,
                    }
                }
            }

            return digit_line;
        })
        .collect::<Vec<_>>();

    let first_and_last_digit_lines = digit_lines
        .iter()
        .map(|line| {
            return format!(
                "{}{}",
                line.chars().next().unwrap(),
                line.chars().last().unwrap()
            );
        })
        .collect::<Vec<_>>();

    first_and_last_digit_lines
        .iter()
        .fold(0, |acc, x| acc + x.parse::<u32>().unwrap())
}

fn digitized(s: &str) -> Option<char> {
    match s {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn combinations(chars: &[char]) -> Vec<String> {
    let mut combinations = Vec::new();

    for start in 0..chars.len() {
        for end in start + 1..=chars.len() {
            combinations.push(chars[start..end].iter().collect());
        }
    }

    combinations
}

fn main() {
    let input = include_str!("../../input.txt");

    println!("{}", calibration_sum(input))
}
