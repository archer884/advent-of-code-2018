mod circle;

use crate::circle::Circle;
use hashbrown::HashMap;
use structopt::StructOpt;

#[derive(Copy, Clone, Debug, StructOpt)]
struct Opt {
    pub players: u32,
    pub marbles: u32,
}

fn main() {
    let Opt { players, marbles } = Opt::from_args();

    println!("Winning score: {}", score(players, marbles));
}

fn score(players: u32, marbles: u32) -> u32 {
    let mut circle = Circle::default();
    let mut scores_by_player = HashMap::new();

    let moves = (0..=marbles).zip((1..=players).cycle());

    for (marble, player) in moves {
        match marble % 23 {
            0 if marble != 0 => {
                *scores_by_player.entry(player).or_insert(0) += marble + circle.pop().unwrap_or(0);
            }

            _ => circle.push(marble),
        }
    }

    scores_by_player.values().cloned().max().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    #[test]
    fn case_10_1618() {
        let actual = super::score(10, 1618);
        assert_eq!(8317, actual, "score( {}, {} )", 10, 1618);
    }

    #[test]
    fn case_13_7999() {
        let actual = super::score(13, 7999);
        assert_eq!(146373, actual, "score( {}, {} )", 13, 7999);
    }

    #[test]
    fn case_17_1104() {
        let actual = super::score(17, 1104);
        assert_eq!(2764, actual, "score( {}, {} )", 17, 1104);
    }

    #[test]
    fn case_21_6111() {
        let actual = super::score(21, 6111);
        assert_eq!(54718, actual, "score( {}, {} )", 21, 6111);
    }

    #[test]
    fn case_30_5807() {
        let actual = super::score(30, 5807);
        assert_eq!(37305, actual, "score( {}, {} )", 30, 5807);
    }
}
