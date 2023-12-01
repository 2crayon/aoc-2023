fn calibration_sum(input: &str) -> u32 {
    let digit_lines = input
        .lines()
        .map(|line| {
            let mut digit_line = String::new();
            for c in line.chars() {
                if c.is_ascii_digit() {
                    digit_line.push(c)
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

fn main() {
    let input = include_str!("../../input.txt");

    println!("{}", calibration_sum(input))
}
