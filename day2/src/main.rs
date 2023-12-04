use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    max_colors: HashMap<String, i32>,
}

fn parse_game(s: &str) -> Game {
    let mut parts = s.split(": ");
    let id = parts.next().unwrap().split(" ").nth(1).unwrap().parse().unwrap();
    let draws = parts.next().unwrap().split("; ");
    let draw_colors = draws.flat_map(|draw| {
        let parts = draw.split(", ");
        parts.map(|part| {
            let mut color_parts = part.split(" ");
            let count = color_parts.next().unwrap().parse().unwrap();
            let color = color_parts.next().unwrap();
            (color.to_string(), count)
        })
    });
    let max_colors = draw_colors.fold(HashMap::new(), |mut acc, (color, count)| {
        let max = acc.entry(color).or_insert(0);
        if count > *max {
            *max = count;
        }
        acc
    });

    Game {
        id,
        max_colors,
    }
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

    let input = BufReader::new(std::fs::File::open(args.input)?);

    let games = input.lines().map(|line| {
        let line = line.unwrap();
        parse_game(&line)
    });

    if !args.part2 {
        let possible_colors = vec![("red", 12), ("green", 13), ("blue", 14)];
        let ids = games.filter(|game| {
            possible_colors.iter().all(|(color, count)| {
                let max = game.max_colors.get(*color).unwrap_or(&0);
                max <= count
            })
        }).map(|game| game.id);

        println!{"{:?}", ids.sum::<u32>()}
    }
    else {
        let powers = games.map(|game| {
            game.max_colors.values().fold(1, |acc, count| {
                acc * count
            })
        });
        println!("{:?}", powers.sum::<i32>());
    }


    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let cases = vec![
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
             Game{id: 1, max_colors: vec![("blue", 6), ("red", 4), ("green", 2)]
                 .iter()
                 .map(|(k, v)| {
                     (k.to_string(), *v)
                 })
                 .collect()
             }),
        ];

        for (input, expected) in cases {
            assert_eq!(parse_game(input), expected);
        }
    }
}