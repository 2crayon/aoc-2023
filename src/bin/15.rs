fn main() {
    let input = include_str!("../../input.txt");

    let sln = input.split(',').map(|s| hash(s)).sum::<usize>();

    println!("{:#?}", sln);
}

fn hash(s: &str) -> usize {
    let mut h = 0;

    for b in s.as_bytes() {
        h += *b as usize;
        h *= 17;
        h %= 256;
    }

    h
}
