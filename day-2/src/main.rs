#[repr(u8)]
#[derive(Clone, Copy)]
enum Throw {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Throw {
    fn from_letter(c: char) -> Throw {
        match c {
            'A' | 'X' => Throw::Rock,
            'B' | 'Y' => Throw::Paper,
            'C' | 'Z' => Throw::Scissors,
            _ => panic!("Invalid letter!"),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum ThrowResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl ThrowResult {
    fn from_letter(c: char) -> ThrowResult {
        match c {
            'X' => ThrowResult::Lose,
            'Y' => ThrowResult::Draw,
            'Z' => ThrowResult::Win,
            _ => panic!("Invalid letter!"),
        }
    }

    fn new(you: Throw, opp: Throw) -> ThrowResult {
        match (you, opp) {
            (Throw::Rock, Throw::Paper)
            | (Throw::Paper, Throw::Scissors)
            | (Throw::Scissors, Throw::Rock) => ThrowResult::Lose,
            (Throw::Rock, Throw::Rock)
            | (Throw::Paper, Throw::Paper)
            | (Throw::Scissors, Throw::Scissors) => ThrowResult::Draw,
            (Throw::Rock, Throw::Scissors)
            | (Throw::Paper, Throw::Rock)
            | (Throw::Scissors, Throw::Paper) => ThrowResult::Win,
        }
    }

    fn requires(self, opp: Throw) -> Throw {
        match (self, opp) {
            (ThrowResult::Win, Throw::Scissors)
            | (ThrowResult::Draw, Throw::Rock)
            | (ThrowResult::Lose, Throw::Paper) => Throw::Rock,
            (ThrowResult::Win, Throw::Rock)
            | (ThrowResult::Draw, Throw::Paper)
            | (ThrowResult::Lose, Throw::Scissors) => Throw::Paper,
            (ThrowResult::Win, Throw::Paper)
            | (ThrowResult::Draw, Throw::Scissors)
            | (ThrowResult::Lose, Throw::Rock) => Throw::Scissors,
        }
    }

    fn score(self, throw: Throw) -> u8 {
        self as u8 + throw as u8
    }
}

fn main() {
    // part 1
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|line| {
            let chars: Vec<_> = line.chars().take(3).collect();
            let opp = Throw::from_letter(chars[0]);
            let you = Throw::from_letter(chars[2]);
            ThrowResult::new(you, opp).score(you) as usize
        })
        .sum();
    println!("sum = {:?}", sum);

    // part 2
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|line| {
            let chars: Vec<_> = line.chars().take(3).collect();
            let opp = Throw::from_letter(chars[0]);
            let result = ThrowResult::from_letter(chars[2]);
            let you = result.requires(opp);
            result.score(you) as usize
        })
        .sum();
    println!("correct sum = {:?}", sum);
}
