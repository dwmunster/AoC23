use std::fs;
use std::io::{BufRead, BufReader};
use clap::Parser;
use indicatif::ProgressStyle;

// Return the first and last digit found in the string
fn first_and_last_digit(s: &str) -> Option<(u32, u32)> {
    let mut res = None;
    for c in s.chars() {
        if c.is_digit(10) {
            let d = c.to_digit(10).unwrap();
            res = match res {
                None => Some((d, d)),
                Some((first, _)) => Some((first, d)),
            }
        }
    }
    res
}

const PART2_TARGETS: [&[char]; 9] = [
    &['o', 'n', 'e'],
    &['t', 'w', 'o'],
    &['t', 'h', 'r', 'e', 'e'],
    &['f', 'o', 'u', 'r'],
    &['f', 'i', 'v', 'e'],
    &['s', 'i', 'x'],
    &['s', 'e', 'v', 'e', 'n'],
    &['e', 'i', 'g', 'h', 't'],
    &['n', 'i', 'n', 'e'],
];

#[derive(Debug)]
enum MatchState {
    NoMatch,
    Match(u32),
    Continue,
}
#[derive(Debug)]
struct State<'a>{
    target: &'a [char],
    value: u32,
    idx: usize,
}

impl<'a> State<'a> {
    fn new(target: &[char], value: u32, idx: usize) -> State {
        State {
            target,
            value,
            idx,
        }
    }

    fn step(&mut self, c: char) -> MatchState {
        if self.idx >= self.target.len() {
            return MatchState::NoMatch;
        }
        if self.target[self.idx] == c {
            self.idx += 1;
            if self.idx >= self.target.len() {
                return MatchState::Match(self.value);
            }
            return MatchState::Continue;
        }
        MatchState::NoMatch
    }
}

fn part2_step(c: char, state: &mut Vec<State>) -> Option<u32> {
    if c.is_ascii_digit() {
        state.clear();
        return Some(c.to_digit(10).unwrap());
    }
    // Advance all current states
    let mut res = None;
    let mut tmp: Vec<State> = Vec::new();
    while let Some(mut s) = state.pop() {
        match s.step(c) {
            MatchState::Match(d) => {
                res = Some(d);
            },
            MatchState::Continue => {
                tmp.push(s);
            },
            MatchState::NoMatch => {},
        }
    }
    // Put back the states that are still valid
    state.extend(tmp.into_iter());

    // Start new states
    for (idx, target) in PART2_TARGETS.iter().enumerate() {
        if target[0] == c {
            state.push(State::new(target, idx as u32 + 1, 1));
        }
    }


    res
}

fn part2_digit(s: &str) -> Option<(u32, u32)> {
    let mut res = None;
    let mut state: Vec<State> = Vec::with_capacity(16);

    for c in s.chars() {
        if let Some(d) = part2_step(c, &mut state) {
            res = match res {
                None => Some((d, d)),
                Some((first, _)) => Some((first, d)),
            }
        }
    }
    res
}


#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    part2: bool,
}


fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Check if we need to use the part22 algorithm
    let parser = if args.part2 {
        part2_digit
    } else {
        first_and_last_digit
    };

    // Set up a progress bar
    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg} ({per_sec})")?
        .progress_chars("#>-"));

    // Open the file and read it line by line
    let res = BufReader::new(fs::File::open(args.input)?)
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, line| {
            pb.inc(1);
            let (first, last) = parser(&line).unwrap_or((0, 0));
            acc + 10*first + last
        });
    pb.finish();
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_and_last_digit() {
        let cases = vec![
            ("abc", None),
            ("a1b2c3", Some((1, 3))),
            ("a123b", Some((1, 3))),
            ("a1b23", Some((1, 3))),
            ("a123b456", Some((1, 6))),
            ("a1b2c3d4e5f6g7h8i9j10k", Some((1, 0))),
            ("treb7uchet", Some((7, 7))),
            ("", None),
            ("1234567890", Some((1, 0)))
        ];
        for (s, expected) in cases {
            assert_eq!(first_and_last_digit(s), expected, "first_and_last_digit({:?}) = {:?}", s, first_and_last_digit(s));
        }
    }

    #[test]
    fn test_part2_digits() {
        let cases = vec![
            ("one23", Some((1, 3))),
            ("ninine", Some((9,9))),
            ("eightwothree", Some((8, 3))),
            ("eightwo", Some((8, 2))),
            ("abcone2threexyz", Some((1, 3))),
            ("7pqrstsixteen", Some((7, 6)))
        ];

        for (s, expected) in cases {
            let actual= part2_digit(s);
            assert_eq!(actual, expected, "day2_digit({:?}) = {:?} != {:?}", s, actual, expected);
        }
    }


}