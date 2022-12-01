fn main() {
    let mut counts = include_str!("input.txt")
        .lines()
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    counts.sort();
    println!("max: {}", counts.last().unwrap());
    println!("top 3: {}", counts.iter().rev().take(3).sum::<usize>());
}
